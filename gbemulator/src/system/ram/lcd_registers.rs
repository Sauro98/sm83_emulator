use super::{default_memory_register_trait_impl, MemoryRegister};

const LCD_CONTROL_REGISTER_ADDRESS: u16 = 0xFF40;
const LCD_STATUS_REGISTER_ADDRESS: u16 = 0xFF41;
const LCD_SCROLL_Y_ADDRESS: u16 = 0xFF42;
const LCD_SCROLL_X_ADDRESS: u16 = 0xFF43;
const LY_REGISTER_ADDRESS: u16 = 0xFF44;
const BG_PALETTE_ADDRESS: u16 = 0xFF47;

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
        (self.value & 0x08) >> 3
    }

    pub fn get_bg_window_tiledata_address(&self) -> u8 {
        (self.value & 0x10) >> 4
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
        [
            self.palette_color_0(),
            self.palette_color_1(),
            self.palette_color_2(),
            self.palette_color_3(),
        ]
    }
}

default_memory_register_trait_impl!(LCDStatusRegister, 0x00);
default_memory_register_trait_impl!(LCDControlRegister, 0x00);
default_memory_register_trait_impl!(LYRegister, 0x00);
default_memory_register_trait_impl!(ScrollXRegister, 0x00);
default_memory_register_trait_impl!(ScrollYRegister, 0x00);
default_memory_register_trait_impl!(BGPaletteRegister, 0x00);

mod test {
    #[test]
    fn test_lcd_control_register() {
        let lcd_control_register = super::LCDControlRegister {
            address: super::LCD_CONTROL_REGISTER_ADDRESS,
            value: 0x91,
        };
        assert_eq!(lcd_control_register.get_lcd_display_enable(), true);
        assert_eq!(lcd_control_register.get_bg_window_tiledata_address(), 0x01);
        assert_eq!(lcd_control_register.get_bg_table_address(), 0x00);
        assert_eq!(lcd_control_register.get_bg_display_enable(), true);
    }
}
