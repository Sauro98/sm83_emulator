use super::clock::SystemClock;
use super::ram::{LCDControlRegister, LCDStatusRegister, LYRegister, MemoryRegister, RAM};

const VBLANK_PERIOD: std::time::Duration = std::time::Duration::from_millis(16);

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

    pub fn get_mode_duration(&self) -> u16 {
        match self {
            LCDMode::HBLANK => 201,
            LCDMode::VBLANK => 4560,
            LCDMode::OAM => 77,
            LCDMode::TX => 169,
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
            LCDMode::VBLANK => 144,
            _ => 0,
        }
    }
}

struct LCDStateMachine {
    active_mode: LCDMode,
    curr_mode_count: u16,
    last_vblank: std::time::Instant,
}

impl LCDStateMachine {
    pub fn new() -> Self {
        LCDStateMachine {
            active_mode: LCDMode::OAM,
            curr_mode_count: 0,
            last_vblank: std::time::Instant::now(),
        }
    }

    pub fn next(&mut self) {
        match self.active_mode {
            LCDMode::VBLANK => {
                if self.curr_mode_count >= self.active_mode.get_mode_duration() {
                    self.last_vblank = std::time::Instant::now();
                    //println!("End VBLANK");
                }
            }
            _ => {
                if self.last_vblank.elapsed() > VBLANK_PERIOD {
                    self.active_mode = LCDMode::VBLANK;
                    //println!("Start VBLANK");
                    self.curr_mode_count = 0;
                }
            }
        }

        if self.curr_mode_count >= self.active_mode.get_mode_duration() {
            self.active_mode = self.active_mode.get_next_state();
            self.curr_mode_count = 0;
        }

        self.curr_mode_count += 1;
    }

    pub fn get_active_mode(&self) -> &LCDMode {
        &self.active_mode
    }
}

pub struct LCDController {
    lcd_control_register: LCDControlRegister,
    lcd_status_register: LCDStatusRegister,
    ly_register: LYRegister,
    clock: SystemClock,
    state_machine: LCDStateMachine,
}

impl LCDController {
    pub fn new() -> Self {
        LCDController {
            lcd_control_register: LCDControlRegister::new(),
            lcd_status_register: LCDStatusRegister::new(),
            ly_register: LYRegister::new(),
            clock: SystemClock::from_frequency(1e6),
            state_machine: LCDStateMachine::new(),
        }
    }

    pub fn next(&mut self, ram: &mut RAM) {
        /*let prev_lcd_enable = self.lcd_control_register.get_lcd_display_enable();
        let prev_window_enable = self.lcd_control_register.get_window_display_enable();
        let prev_sprite_enable = self.lcd_control_register.get_sprite_display_enable();
        let prev_bg_enable = self.lcd_control_register.get_bg_display_enable();*/
        self.read_from_ram(ram);
        /*let curr_lcd_enable = self.lcd_control_register.get_lcd_display_enable();
        let curr_window_enable = self.lcd_control_register.get_window_display_enable();
        let curr_sprite_enable = self.lcd_control_register.get_sprite_display_enable();
        let curr_bg_enable = self.lcd_control_register.get_bg_display_enable();*/

        /*if curr_lcd_enable != prev_lcd_enable {
            println!(
                "LCD enable switched from {} to {}",
                prev_lcd_enable, curr_lcd_enable
            );
        }

        if curr_window_enable != prev_window_enable {
            println!(
                "Window enable switched from {} to {}",
                prev_window_enable, curr_window_enable
            );
        }

        if curr_sprite_enable != prev_sprite_enable {
            println!(
                "Sprite enable switched from {} to {}",
                prev_sprite_enable, curr_sprite_enable
            );
        }

        if curr_bg_enable != prev_bg_enable {
            println!(
                "BG enable switched from {} to {}",
                prev_bg_enable, curr_bg_enable
            );
        }*/
        self.state_machine.next();
        self.lcd_status_register
            .set_status(self.state_machine.get_active_mode().get_status_byte());
        self.ly_register
            .set_line(self.state_machine.get_active_mode().get_vertical_line());
        self.load_in_ram(ram);
        self.clock.next();
    }
}

impl MemoryRegister for LCDController {
    fn reset(&mut self) {
        self.lcd_control_register.reset();
        self.lcd_status_register.reset();
        self.ly_register.reset();
    }

    fn load_in_ram(&self, ram: &mut super::ram::RAM) -> Option<()> {
        let option_control = self.lcd_control_register.load_in_ram(ram);
        let option_status = self.lcd_status_register.load_in_ram(ram);
        let option_ly = self.ly_register.load_in_ram(ram);
        if option_control.is_some() && option_status.is_some() && option_ly.is_some() {
            option_control
        } else {
            None
        }
    }

    fn read_from_ram(&mut self, ram: &super::ram::RAM) {
        self.lcd_control_register.read_from_ram(ram);
        self.lcd_status_register.read_from_ram(ram);
        self.ly_register.read_from_ram(ram);
    }
}
