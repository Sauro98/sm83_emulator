use crate::system::ram::default_nonimplemented_memory_register_trait_impl;
use crate::system::ram::lcd_registers::{
    BGPaletteRegister, LCDControlRegister, LCDStatusRegister, LYRegister, ScrollXRegister,
    ScrollYRegister,
};
use crate::system::ram::{MemoryRegister, RAM};
use show_image::{create_window, ImageInfo, ImageView};
use std::sync::{Arc, Mutex};
use std::time::Instant;

const VBLANK_PERIOD: std::time::Duration = std::time::Duration::from_millis(16);
const BG_TILEMAP_SELECT_ADDRESSES: [u16; 2] = [0x9800, 0x9C00];
const BG_WINDOW_TILEDATA_SELECT_ADDRESSES: [u16; 2] = [0x8800, 0x8000];
const BG_SHADES: [u8; 4] = [255, 192, 64, 0];
const GB_SCREEN_WIDTH: usize = 160;
const GB_SCREEN_HEIGHT: usize = 144;

#[derive(Clone)]
enum LCDMode {
    HBLANK,
    VBLANK,
    OAM,
    TX,
}

impl LCDMode {
    pub fn get_status_byte(&self) -> u8 {
        match self {
            LCDMode::HBLANK => 0x08,
            LCDMode::VBLANK => 0x11,
            LCDMode::OAM => 0x22,
            LCDMode::TX => 0x03,
        }
    }

    pub fn get_mode_duration(&self) -> std::time::Duration {
        match self {
            LCDMode::HBLANK => std::time::Duration::from_nanos(48600),
            LCDMode::VBLANK => std::time::Duration::from_micros(1080),
            LCDMode::OAM => std::time::Duration::from_micros(19),
            LCDMode::TX => std::time::Duration::from_micros(41),
        }
    }

    pub fn get_line_duration(&self) -> std::time::Duration {
        match self {
            LCDMode::VBLANK => LCDMode::get_mode_duration(&LCDMode::VBLANK) / 30,
            _ => {
                (LCDMode::get_mode_duration(&LCDMode::HBLANK)
                    + LCDMode::get_mode_duration(&LCDMode::OAM)
                    + LCDMode::get_mode_duration(&LCDMode::TX))
                    / 144
            }
        }
    }

    pub fn get_next_state(&self) -> Self {
        match self {
            LCDMode::HBLANK => LCDMode::OAM,
            LCDMode::VBLANK => LCDMode::OAM,
            LCDMode::OAM => LCDMode::TX,
            LCDMode::TX => LCDMode::HBLANK,
        }
    }

    pub fn get_vertical_line(&self) -> u8 {
        match self {
            LCDMode::VBLANK => 153 as u8,
            _ => 0,
        }
    }
}

struct LCDStateMachine {
    active_mode: LCDMode,
    curr_mode_start: std::time::Instant,
    last_vblank: std::time::Instant,
    current_line: u8,
    last_line_time: std::time::Instant,
}

impl LCDStateMachine {
    pub fn new() -> Self {
        LCDStateMachine {
            active_mode: LCDMode::OAM,
            curr_mode_start: std::time::Instant::now(),
            last_vblank: std::time::Instant::now(),
            current_line: 0,
            last_line_time: std::time::Instant::now(),
        }
    }

    pub fn next(&mut self) {
        match self.active_mode {
            LCDMode::VBLANK => {
                if self.curr_mode_start.elapsed() >= self.active_mode.get_mode_duration() {
                    self.last_vblank = std::time::Instant::now();
                    self.current_line = 0;
                    self.last_line_time = Instant::now();
                    //println!("End VBLANK");
                } else if self.last_line_time.elapsed() > self.active_mode.get_line_duration() {
                    self.current_line = if self.current_line < 153 {
                        self.current_line + 1
                    } else {
                        self.current_line
                    };
                    self.last_line_time = Instant::now();
                }
            }
            _ => {
                if self.last_vblank.elapsed() > VBLANK_PERIOD {
                    self.active_mode = LCDMode::VBLANK;
                    //println!("Start VBLANK");
                    self.curr_mode_start = std::time::Instant::now();
                    self.current_line = GB_SCREEN_HEIGHT as u8;
                    self.last_line_time = Instant::now();
                } else if self.last_line_time.elapsed() > self.active_mode.get_line_duration() {
                    self.current_line = if self.current_line < GB_SCREEN_HEIGHT as u8 - 1 {
                        self.current_line + 1
                    } else {
                        self.current_line
                    };
                    self.last_line_time = Instant::now()
                }
            }
        }

        if self.curr_mode_start.elapsed() >= self.active_mode.get_mode_duration() {
            self.active_mode = self.active_mode.get_next_state();
            self.curr_mode_start = std::time::Instant::now();
        }
    }

    pub fn get_active_mode(&self) -> &LCDMode {
        &self.active_mode
    }

    pub fn get_current_line(&self) -> u8 {
        self.current_line
    }
}

#[derive(Copy, Clone)]
struct BGTile {
    pub pixel_data: [u8; 8 * 8],
}

impl BGTile {
    pub fn new() -> Self {
        BGTile {
            pixel_data: [0u8; 8 * 8],
        }
    }

    pub fn read_from_ram(
        ram: &RAM,
        tile_map_address: u16,
        tile_data_address: u16,
        index: u16,
        palette: &BGPaletteRegister,
    ) -> Self {
        let palette_colors = palette.palette_colors();
        // if palette_colors[1] != 0 {
        //     println!("{:?}", palette_colors);
        // }
        let mut tile = BGTile::new();
        let tile_index = ram.get_at(tile_map_address + index).unwrap();
        /*if (tile_map_address + index) == 0x9910 {
            println!(
                "tile address {:X} tile index {}",
                tile_map_address + index,
                tile_index
            );
        }*/
        let tile_data =
            ram.get_tile_data(tile_data_address, tile_index, tile_data_address == 0x8800);
        for row in 0..8 {
            for bit in 0..8 {
                tile.pixel_data[row * 8 + bit] = ((tile_data[row * 2] >> (7 - bit)) & 0x01)
                    | (((tile_data[row * 2 + 1] >> (7 - bit)) & 0x01) << 1);
            }
            for bit in 0..8 {
                tile.pixel_data[row * 8 + bit] =
                    BG_SHADES[palette_colors[tile.pixel_data[row * 8 + bit] as usize]]
            }
        }

        // for i in 0..(8 * 8) {
        //     tile.pixel_data[i] = tile_index;
        // }
        tile
    }
}

struct LCDImage {
    x_pos: u8,
    y_pos: u8,
    scroll_x: ScrollXRegister,
    scroll_y: ScrollYRegister,
    bg_palette: BGPaletteRegister,
    tile_map: [BGTile; 32 * 32],
    pixel_data: [u8; 256 * 256],
    bg_tilemap_address: u16,
    bg_win_tiledata_address: u16,
}

impl LCDImage {
    pub fn new() -> Self {
        LCDImage {
            x_pos: 0,
            y_pos: 0,
            scroll_x: ScrollXRegister::new(),
            scroll_y: ScrollYRegister::new(),
            bg_palette: BGPaletteRegister::new(),
            tile_map: [BGTile::new(); 32 * 32],
            pixel_data: [0u8; 256 * 256],
            bg_tilemap_address: BG_TILEMAP_SELECT_ADDRESSES[0],
            bg_win_tiledata_address: BG_WINDOW_TILEDATA_SELECT_ADDRESSES[0],
        }
    }

    pub fn reset(&mut self) {
        for pix in self.pixel_data.iter_mut() {
            *pix = 255;
        }
    }

    pub fn get_data(&self) -> [u8; GB_SCREEN_HEIGHT * GB_SCREEN_WIDTH] {
        let mut data = [255u8; GB_SCREEN_HEIGHT * GB_SCREEN_WIDTH];
        for i in 0i32..(255 * 255) {
            let row = i / 256;
            let col = i % 256;
            let data_row = row - self.y_pos as i32;
            let data_col = col - self.x_pos as i32;
            if data_col >= 0
                && data_col < GB_SCREEN_WIDTH as i32
                && data_row >= 0
                && data_row < GB_SCREEN_HEIGHT as i32
            {
                data[data_row as usize * GB_SCREEN_WIDTH + data_col as usize] =
                    self.pixel_data[row as usize * 256 + col as usize];
            }
        }
        data
    }

    pub fn set_pos(&mut self, x_pos: u8, y_pos: u8) {
        if x_pos < 111 {
            self.x_pos = x_pos;
        } else {
            self.x_pos = 0;
        }

        if y_pos < 95 {
            self.y_pos = y_pos;
        } else {
            self.y_pos = 0;
        }
    }

    pub fn set_bg_vram_address(&mut self, address: u16) {
        if address != self.bg_tilemap_address && address == 0x9800 {
            println!("setting tilemap 1")
        }
        if address != self.bg_tilemap_address && address == 0x9C00 {
            println!("setting tilemap 2")
        }
        self.bg_tilemap_address = address;
    }

    pub fn set_bg_win_tiledata_address(&mut self, address: u16) {
        if address != self.bg_win_tiledata_address && address == 0x8800 {
            println!("setting negative offset bg tiledata")
        }
        if address != self.bg_win_tiledata_address && address == 0x8000 {
            println!("setting postive offset bg tiledata")
        }
        self.bg_win_tiledata_address = address;
    }

    pub fn read_tilemap(&mut self, ram: &RAM) {
        for i in 0..self.tile_map.len() {
            self.tile_map[i] = BGTile::read_from_ram(
                ram,
                self.bg_tilemap_address,
                self.bg_win_tiledata_address,
                i as u16,
                &self.bg_palette,
            );
        }
    }

    pub fn draw(&mut self, draw_bg: bool, draw_window: bool, draw_sprites: bool) {
        /*for i in 0i32..(255 * 255) {
            let row = i / 256;
            let col = i % 256;
            let tile_row = row / 8;
            let tile_col = col / 8;
            let tile_index = tile_row * 32 + tile_col;
            let tile_pixel_row = row % 8;
            let tile_pixel_col = col % 8;
            self.pixel_data[i as usize] = self.tile_map[tile_index as usize].pixel_data
                [(tile_pixel_row * 8 + tile_pixel_col) as usize];
        }*/
        if draw_bg {
            //println!("draw bg");
            for tile_row in 0..32 {
                for tile_col in 0..32 {
                    let curr_tile = &self.tile_map[tile_row * 32 + tile_col];
                    for tile_pixel_row in 0..8 {
                        for tile_pixel_col in 0..8 {
                            self.pixel_data[(((tile_row * 8) + tile_pixel_row) * 256
                                + ((tile_col * 8) + tile_pixel_col))
                                as usize] =
                                curr_tile.pixel_data[tile_pixel_row * 8 + tile_pixel_col]
                        }
                    }
                }
            }
        } else {
            for i in 0..self.pixel_data.len() {
                self.pixel_data[i] = 255;
            }
        }
        if draw_window {
            println!("draw window");
        }
        if draw_sprites {
            println!("draw_sprites");
        }
    }
}

impl MemoryRegister for LCDImage {
    fn reset(&mut self) {
        self.scroll_x.reset();
        self.scroll_x.reset();
        self.bg_palette.reset();
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        self.scroll_x.load_in_ram(ram);
        self.scroll_y.load_in_ram(ram);
        self.bg_palette.load_in_ram(ram)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.scroll_x.read_from_ram(ram);
        self.scroll_y.read_from_ram(ram);
        self.set_pos(self.scroll_x.value, self.scroll_y.value);
        self.bg_palette.read_from_ram(ram);
    }

    default_nonimplemented_memory_register_trait_impl!();
}

pub struct LCDController {
    lcd_control_register: LCDControlRegister,
    lcd_status_register: LCDStatusRegister,
    ly_register: LYRegister,
    state_machine: LCDStateMachine,
    pixel_data: Arc<Mutex<LCDImage>>,
    thread_finished: Arc<Mutex<bool>>,
    image_ready: Arc<Mutex<bool>>,
    thread_handle: std::thread::JoinHandle<()>,
    should_draw: bool,
}

impl LCDController {
    pub fn new() -> Self {
        let thread_finished = std::sync::Arc::new(std::sync::Mutex::new(false));
        let image_ready = std::sync::Arc::new(std::sync::Mutex::new(false));
        let pixel_data = std::sync::Arc::new(std::sync::Mutex::new(LCDImage::new()));
        LCDController {
            lcd_control_register: LCDControlRegister::new(),
            lcd_status_register: LCDStatusRegister::new(),
            ly_register: LYRegister::new(),
            state_machine: LCDStateMachine::new(),
            pixel_data: pixel_data.clone(),
            thread_finished: thread_finished.clone(),
            image_ready: image_ready.clone(),
            thread_handle: std::thread::spawn(move || {
                let thread_finished_ref = thread_finished.clone();
                let image_ready_ref = image_ready.clone();
                let pixel_data_ref = pixel_data.clone();
                let display_window = create_window("GameBoy Screen", Default::default()).unwrap();
                let mut prev_cycle_time = std::time::Instant::now();
                loop {
                    if *thread_finished_ref.lock().unwrap() {
                        break;
                    }
                    {
                        let mut ready = image_ready_ref.lock().unwrap();
                        if *ready {
                            let current_data = pixel_data_ref.lock().unwrap().get_data();
                            let image = ImageView::new(
                                ImageInfo::mono8(GB_SCREEN_WIDTH as u32, GB_SCREEN_HEIGHT as u32),
                                &current_data,
                            );
                            let fps = 1e9 / prev_cycle_time.elapsed().as_nanos() as f64;
                            display_window
                                .set_image(format!("GameBoy Screen {:.2} fps", fps), image)
                                .unwrap();
                            //println!("Drawing {:.2}fps", fps);
                            prev_cycle_time = std::time::Instant::now();
                            *ready = false;
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(2));
                }
                println!("outside display loop")
            }),
            should_draw: false,
        }
    }

    pub fn next(&mut self, ram: &Arc<Mutex<RAM>>) {
        let mut ram = ram.lock().unwrap();
        self.read_from_ram(&ram);

        if !self.lcd_control_register.get_lcd_display_enable() {
            self.pixel_data.lock().unwrap().reset();
            *self.image_ready.lock().unwrap() = true;
        } else {
            {
                let mut pixel_data_ref = self.pixel_data.lock().unwrap();
                pixel_data_ref.read_from_ram(&ram);
                pixel_data_ref.set_bg_vram_address(
                    BG_TILEMAP_SELECT_ADDRESSES
                        [self.lcd_control_register.get_bg_table_address() as usize],
                );
                pixel_data_ref.set_bg_win_tiledata_address(
                    BG_WINDOW_TILEDATA_SELECT_ADDRESSES
                        [self.lcd_control_register.get_bg_window_tiledata_address() as usize],
                );
            }

            match self.state_machine.get_active_mode() {
                LCDMode::TX => {
                    self.pixel_data.lock().unwrap().read_tilemap(&ram);
                }
                LCDMode::VBLANK => {
                    if self.should_draw {
                        self.pixel_data.lock().unwrap().draw(
                            self.lcd_control_register.get_bg_display_enable(),
                            self.lcd_control_register.get_window_display_enable(),
                            self.lcd_control_register.get_sprite_display_enable(),
                        );
                        *self.image_ready.lock().unwrap() = true;
                        self.should_draw = false;
                    }
                }
                LCDMode::OAM => {
                    self.should_draw = true;
                }
                _ => {}
            }
            self.state_machine.next();
            self.lcd_status_register
                .set_status(self.state_machine.get_active_mode().get_status_byte());
            self.ly_register
                .set_line(self.state_machine.get_current_line());
            self.load_in_ram(&mut ram);
            self.pixel_data.lock().unwrap().load_in_ram(&mut ram);
        }
    }

    pub fn stop_window_thread(&self) {
        let mut reference = self.thread_finished.lock().unwrap();
        *reference = true;
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}

impl MemoryRegister for LCDController {
    fn reset(&mut self) {
        self.lcd_control_register.reset();
        self.lcd_status_register.reset();
        self.ly_register.reset();
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        let option_control = self.lcd_control_register.load_in_ram(ram);
        let option_status = self.lcd_status_register.load_in_ram(ram);
        let option_ly = self.ly_register.load_in_ram(ram);
        if option_control.is_some() && option_status.is_some() && option_ly.is_some() {
            option_control
        } else {
            None
        }
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.lcd_control_register.read_from_ram(ram);
        self.lcd_status_register.read_from_ram(ram);
        self.ly_register.read_from_ram(ram);
    }

    default_nonimplemented_memory_register_trait_impl!();
}
