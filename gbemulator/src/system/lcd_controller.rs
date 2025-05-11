use super::ram::{LCDControlRegister, LCDStatusRegister, MemoryRegister};

pub struct LCDController {
    lcd_control_register: LCDControlRegister,
    lcd_status_register: LCDStatusRegister,
}

impl LCDController {
    pub fn new() -> Self {
        LCDController {
            lcd_control_register: LCDControlRegister::new(),
            lcd_status_register: LCDStatusRegister::new(),
        }
    }
}

impl MemoryRegister for LCDController {
    fn reset(&mut self) {
        self.lcd_control_register.reset();
        self.lcd_status_register.reset();
    }

    fn load_in_ram(&self, ram: &mut super::ram::RAM) -> Option<()> {
        let option_control = self.lcd_control_register.load_in_ram(ram);
        let option_status = self.lcd_status_register.load_in_ram(ram);
        if option_control.is_some() && option_status.is_some() {
            option_control
        } else {
            None
        }
    }

    fn read_from_ram(&mut self, ram: &super::ram::RAM) {
        self.lcd_control_register.read_from_ram(ram);
        self.lcd_status_register.read_from_ram(ram);
    }
}
