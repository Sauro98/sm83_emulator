use std::sync::Mutex;

const BOOT_ROM_END: u16 = 0x00FF;
const BOOTLOCKER_ADDRESS: u16 = 0xFF50;
const BOOTLOCKER_LOCKED: u8 = 0x01;
const BOOTLOCKER_UNLOCKED: u8 = 0x00;
const DMA_ADDRESS: u16 = 0xFF46;
const LCD_CONTROL_REGISTER_ADDRESS: u16 = 0xFF40;
const LCD_STATUS_REGISTER_ADDRESS: u16 = 0xFF41;
const LCD_SCROLL_Y_ADDRESS: u16 = 0xFF42;
const LCD_SCROLL_X_ADDRESS: u16 = 0xFF43;
const LY_REGISTER_ADDRESS: u16 = 0xFF44;
const BG_PALETTE_ADDRESS: u16 = 0xFF47;

pub struct RAM {
    data: std::vec::Vec<u8>,
    #[allow(dead_code)]
    capacity: usize,
    dma_requested: bool,
}

impl Clone for RAM {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            capacity: self.capacity.clone(),
            dma_requested: self.dma_requested.clone(),
        }
    }
}

impl RAM {
    pub fn new() -> RAM {
        let capacity = u16::MAX as usize + 1;
        let data = vec![0; capacity];
        RAM {
            data: data,
            capacity: capacity,
            dma_requested: false,
        }
    }

    pub fn get_at(&self, address: u16) -> Option<u8> {
        self.data.get(address as usize).copied()
    }

    pub fn set_at(&mut self, address: u16, value: u8) -> Option<()> {
        if self.data.get(BOOTLOCKER_ADDRESS as usize) == Some(&BOOTLOCKER_LOCKED) {
            // if bootlock is set
            if address <= BOOT_ROM_END {
                return None; // cannot set boot memory if lock is on
            }
        }
        if address == DMA_ADDRESS {
            self.dma_requested = true;
        }
        match self.data.get_mut(address as usize) {
            Some(x) => *x = value,
            None => return None,
        }
        Some(())
    }

    pub fn was_dma_requested(&self) -> bool {
        self.dma_requested
    }

    pub fn reset_dma_request(&mut self) {
        self.dma_requested = false;
    }

    pub fn get_tile_data(
        &self,
        start_address: u16,
        offset: u8,
        is_offset_negative: bool,
    ) -> [u8; 16] {
        let mut result = [0; 16];
        let mut adjusted_offset = (start_address + (offset as u16) * 16) as usize;
        if is_offset_negative {
            let negative_offset = ((offset as i8) as i32 + 128) * 16;
            adjusted_offset = ((start_address as i32 + negative_offset) as u16) as usize;
        }

        for i in 0..16 {
            result[i] = *self.data.get(adjusted_offset + i).unwrap();
        }
        result
    }
}

pub trait MemoryRegister {
    fn reset(&mut self);
    fn load_in_ram(&self, ram: &mut RAM) -> Option<()>;
    fn read_from_ram(&mut self, ram: &RAM);
}

pub struct BootLockMemoryRegister {
    address: u16,
    value: u8,
}

pub struct BootRom {
    contents: Vec<u8>,
}

impl BootLockMemoryRegister {
    pub fn new() -> Self {
        BootLockMemoryRegister {
            address: BOOTLOCKER_ADDRESS,
            value: BOOTLOCKER_UNLOCKED,
        }
    }

    pub fn lock(&mut self) {
        self.value = BOOTLOCKER_LOCKED;
    }

    pub fn unlock(&mut self) {
        self.value = BOOTLOCKER_UNLOCKED;
    }
}

impl MemoryRegister for BootLockMemoryRegister {
    fn reset(&mut self) {
        self.value = BOOTLOCKER_UNLOCKED;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

impl BootRom {
    pub fn new() -> Self {
        BootRom {
            contents: vec![
                0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26,
                0xff, 0x0e, 0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77,
                0x77, 0x3e, 0xfc, 0xe0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95,
                0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b, 0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06,
                0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9, 0x3e, 0x19, 0xea, 0x10, 0x99, 0x21,
                0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20, 0xf9, 0x2e, 0x0f, 0x18,
                0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04, 0x1e, 0x02,
                0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
                0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64,
                0x20, 0x06, 0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15,
                0x20, 0xd2, 0x05, 0x20, 0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb,
                0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17, 0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9,
                0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c,
                0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6,
                0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc,
                0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
                0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe,
                0x34, 0x20, 0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe,
                0x3e, 0x01, 0xe0, 0x50,
            ],
        }
    }
}

impl MemoryRegister for BootRom {
    fn reset(&mut self) {
        self.contents = vec![0x00; 255];
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        for (address, value) in self.contents.iter().enumerate() {
            let res = ram.set_at(address as u16, value.to_owned());
            if res.is_none() {
                return res;
            }
        }
        Some(())
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        unimplemented!()
    }
}

pub struct LCDControlRegister {
    address: u16,
    value: u8,
}

impl LCDControlRegister {
    pub fn new() -> Self {
        LCDControlRegister {
            address: LCD_CONTROL_REGISTER_ADDRESS,
            value: 0x00,
        }
    }

    pub fn get_lcd_display_enable(&self) -> bool {
        self.value & 0x80 > 0
    }

    pub fn get_window_display_enable(&self) -> bool {
        self.value & 0x20 > 0
    }

    pub fn get_sprite_display_enable(&self) -> bool {
        self.value & 0x02 > 0
    }

    pub fn get_bg_display_enable(&self) -> bool {
        self.value & 0x01 > 0
    }

    pub fn get_bg_table_address(&self) -> u8 {
        self.value & 0x08 >> 3
    }

    pub fn get_bg_window_tiledata_address(&self) -> u8 {
        self.value & 0x10 >> 4
    }
}

impl MemoryRegister for LCDControlRegister {
    fn reset(&mut self) {
        self.value = 0x00;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

pub struct LCDStatusRegister {
    address: u16,
    value: u8,
}

impl LCDStatusRegister {
    pub fn new() -> Self {
        LCDStatusRegister {
            address: LCD_STATUS_REGISTER_ADDRESS,
            value: 0x00,
        }
    }

    pub fn set_status(&mut self, status: u8) {
        self.value = status;
    }
}

impl MemoryRegister for LCDStatusRegister {
    fn reset(&mut self) {
        self.value = 0x00;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

pub struct LYRegister {
    address: u16,
    value: u8,
}

impl LYRegister {
    pub fn new() -> Self {
        LYRegister {
            address: LY_REGISTER_ADDRESS,
            value: 0x0,
        }
    }

    pub fn set_line(&mut self, line: u8) {
        self.value = line
    }
}

impl MemoryRegister for LYRegister {
    fn reset(&mut self) {
        self.value = 0x00;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

pub struct ScrollXRegister {
    address: u16,
    pub value: u8,
}
pub struct ScrollYRegister {
    address: u16,
    pub value: u8,
}

impl ScrollXRegister {
    pub fn new() -> Self {
        ScrollXRegister {
            address: LCD_SCROLL_X_ADDRESS,
            value: 0x0,
        }
    }
}

impl ScrollYRegister {
    pub fn new() -> Self {
        ScrollYRegister {
            address: LCD_SCROLL_Y_ADDRESS,
            value: 0x0,
        }
    }
}

impl MemoryRegister for ScrollXRegister {
    fn reset(&mut self) {
        self.value = 0x0;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

impl MemoryRegister for ScrollYRegister {
    fn reset(&mut self) {
        self.value = 0x0;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

pub struct BGPaletteRegister {
    value: u8,
    address: u16,
}

impl BGPaletteRegister {
    pub fn new() -> Self {
        BGPaletteRegister {
            value: 0x0,
            address: BG_PALETTE_ADDRESS,
        }
    }

    pub fn palette_color_0(&self) -> usize {
        (self.value & 0x03) as usize
    }

    pub fn palette_color_1(&self) -> usize {
        ((self.value >> 2) & 0x03) as usize
    }

    pub fn palette_color_2(&self) -> usize {
        ((self.value >> 4) & 0x03) as usize
    }

    pub fn palette_color_3(&self) -> usize {
        ((self.value >> 6) & 0x03) as usize
    }

    pub fn palette_colors(&self) -> [usize; 4] {
        /*if self.value != 0 {
            println!("{}", self.value);
        }*/
        [
            self.palette_color_0(),
            self.palette_color_1(),
            self.palette_color_2(),
            self.palette_color_3(),
        ]
    }
}

impl MemoryRegister for BGPaletteRegister {
    fn reset(&mut self) {
        self.value = 0x0;
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.address, self.value)
    }

    fn read_from_ram(&mut self, ram: &RAM) {
        self.value = ram.get_at(self.address).unwrap();
    }
}

pub struct FakeRom {
    address: u16,
    contents: Vec<u8>,
}

impl FakeRom {
    pub fn new() -> Self {
        FakeRom {
            address: 0x0104,
            contents: vec![
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
        }
    }
}

impl MemoryRegister for FakeRom {
    fn reset(&mut self) {}

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        for i in 0..self.contents.len() {
            ram.set_at(self.address + i as u16, self.contents[i]);
        }
        Some(())
    }

    fn read_from_ram(&mut self, ram: &RAM) {}
}
