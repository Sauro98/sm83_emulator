struct Shared16BitRegister {
    content: u16,
}

impl Shared16BitRegister {
    pub fn new() -> Shared16BitRegister {
        return Shared16BitRegister { content: 0 };
    }

    pub fn first(&self) -> u8 {
        return ((self.content & 0xFF00) >> 8) as u8;
    }

    pub fn second(&self) -> u8 {
        return (self.content & 0x00FF) as u8;
    }

    pub fn write_first(&mut self, value: u8) {
        self.content = self.content & 0x00FF | ((value as u16) << 8);
    }

    pub fn write_second(&mut self, value: u8) {
        self.content = self.content & 0xFF00 | (value as u16);
    }
}

#[allow(dead_code)]
pub enum RegisterName {
    IR,
    IE,
    A,
    F,
    AF,
    B,
    C,
    BC,
    D,
    E,
    DE,
    H,
    L,
    HL,
    PC,
    SP,
}

pub struct RegisterFile {
    irie: Shared16BitRegister,
    af: Shared16BitRegister,
    bc: Shared16BitRegister,
    de: Shared16BitRegister,
    hl: Shared16BitRegister,
    pc: u16,
    sp: Shared16BitRegister,
}

impl RegisterFile {
    pub fn new() -> RegisterFile {
        return RegisterFile {
            irie: Shared16BitRegister::new(),
            af: Shared16BitRegister::new(),
            bc: Shared16BitRegister::new(),
            de: Shared16BitRegister::new(),
            hl: Shared16BitRegister::new(),
            pc: 0,
            sp: Shared16BitRegister::new(),
        };
    }

    pub fn get_ir(&self) -> u8 {
        return self.irie.first();
    }

    pub fn get_ie(&self) -> u8 {
        return self.irie.second();
    }

    pub fn get_af(&self) -> u16 {
        return self.af.content;
    }

    pub fn get_a(&self) -> u8 {
        return self.af.first();
    }

    pub fn get_f(&self) -> u8 {
        return self.af.second();
    }

    pub fn get_bc(&self) -> u16 {
        return self.bc.content;
    }

    pub fn get_b(&self) -> u8 {
        return self.bc.first();
    }

    pub fn get_c(&self) -> u8 {
        return self.bc.second();
    }

    pub fn get_de(&self) -> u16 {
        return self.de.content;
    }

    pub fn get_d(&self) -> u8 {
        return self.de.first();
    }

    pub fn get_e(&self) -> u8 {
        return self.de.second();
    }

    pub fn get_hl(&self) -> u16 {
        return self.hl.content;
    }

    pub fn get_h(&self) -> u8 {
        return self.hl.first();
    }

    pub fn get_l(&self) -> u8 {
        return self.hl.second();
    }

    pub fn get_pc(&self) -> u16 {
        return self.pc;
    }

    pub fn get_sp(&self) -> u16 {
        return self.sp.content;
    }

    pub fn get_s(&self) -> u8 {
        return self.sp.first();
    }

    pub fn get_p(&self) -> u8 {
        return self.sp.second();
    }

    pub fn set_ir(&mut self, value: u8) {
        return self.irie.write_first(value);
    }

    pub fn set_ie(&mut self, value: u8) {
        return self.irie.write_second(value);
    }

    pub fn set_af(&mut self, value: u16) {
        return self.af.content = value;
    }

    pub fn set_a(&mut self, value: u8) {
        return self.af.write_first(value);
    }

    pub fn set_f(&mut self, value: u8) {
        return self.af.write_second(value);
    }

    pub fn set_bc(&mut self, value: u16) {
        return self.bc.content = value;
    }

    pub fn set_b(&mut self, value: u8) {
        return self.bc.write_first(value);
    }

    pub fn set_c(&mut self, value: u8) {
        return self.bc.write_second(value);
    }

    pub fn set_de(&mut self, value: u16) {
        return self.de.content = value;
    }

    pub fn set_d(&mut self, value: u8) {
        return self.de.write_first(value);
    }

    pub fn set_e(&mut self, value: u8) {
        return self.de.write_second(value);
    }

    pub fn set_hl(&mut self, value: u16) {
        return self.hl.content = value;
    }

    pub fn set_h(&mut self, value: u8) {
        return self.hl.write_first(value);
    }

    pub fn set_l(&mut self, value: u8) {
        return self.hl.write_second(value);
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp.content = value;
    }

    pub fn set_s(&mut self, value: u8) {
        self.sp.write_first(value);
    }

    pub fn set_p(&mut self, value: u8) {
        self.sp.write_second(value);
    }

    pub fn set(&mut self, reg: u8, val: u8) -> Result<(), std::string::String> {
        if reg == 0x00 {
            self.set_b(val);
        } else if reg == 0x01 {
            self.set_c(val);
        } else if reg == 0x02 {
            self.set_a(val);
        } else if reg == 0x03 {
            self.set_f(val);
        } else if reg == 0x04 {
            self.set_d(val);
        } else if reg == 0x05 {
            self.set_e(val);
        } else if reg == 0x06 {
            self.set_h(val);
        } else if reg == 0x07 {
            self.set_l(val);
        } else {
            return Err(format!("unrecognized register {}", u8::to_string(&reg)));
        }

        Ok(())
    }

    pub fn get(&self, reg: u8) -> Result<u8, std::string::String> {
        if reg == 0x07 {
            Ok(self.get_a())
        } else if reg == 0x00 {
            Ok(self.get_b())
        } else if reg == 0x01 {
            Ok(self.get_c())
        } else if reg == 0x02 {
            Ok(self.get_d())
        } else if reg == 0x03 {
            Ok(self.get_e())
        } else if reg == 0x04 {
            Ok(self.get_h())
        } else if reg == 0x05 {
            Ok(self.get_l())
        } else {
            Err(format!("unrecognized register {}", u8::to_string(&reg)))
        }
    }

    pub fn get16(&self, reg: u8) -> Result<u16, std::string::String> {
        if reg == 0x00 {
            Ok(self.get_bc())
        } else if reg == 0x01 {
            Ok(self.get_de())
        } else if reg == 0x02 {
            Ok(self.get_hl())
        } else if reg == 0x03 {
            Ok(self.get_sp())
        } else {
            Err(format!("unrecognized register {}", u8::to_string(&reg)))
        }
    }

    pub fn set16(&mut self, reg: u8, val: u16) -> Result<(), std::string::String> {
        if reg == 0x00 {
            self.set_bc(val);
        } else if reg == 0x01 {
            self.set_af(val);
        } else if reg == 0x02 {
            self.set_de(val);
        } else if reg == 0x03 {
            self.set_hl(val);
        } else {
            return Err(format!("unrecognized register {}", u8::to_string(&reg)));
        }

        Ok(())
    }

    pub fn get_zero_flag(&self) -> u8 {
        (self.af.second() & 0x80) >> 7
    }

    pub fn set_negative_flag(&mut self) {
        self.af.write_second(self.af.second() | 0x40);
    }

    pub fn unset_negative_flag(&mut self) {
        self.af.write_second(self.af.second() & 0xBF);
    }

    pub fn get_half_carry_flag(&self) -> u8 {
        (self.af.second() & 0x20) >> 5
    }

    pub fn set_half_carry_flag(&mut self) {
        self.af.write_second(self.af.second() | 0x20);
    }

    pub fn unset_half_carry_flag(&mut self) {
        self.af.write_second(self.af.second() & 0xDF);
    }

    pub fn get_carry_flag(&self) -> u8 {
        (self.af.second() & 0x10) >> 4
    }

    pub fn set_carry_flag(&mut self) {
        self.af.write_second(self.af.second() | 0x10);
    }

    pub fn or_flags(&mut self, flags: u8) {
        self.af
            .write_second((flags & 0xE0) | (self.af.second() & 0x1F));
    }

    pub fn flip_carry_flag(&mut self) {
        // xor with the carry bit mask:
        // the other bits are left untouched since 1^0 = 1 and 0^0=0
        // the carry bit is flipped since 1^0 = 1 and 1^1=0
        self.af.write_second(self.af.second() ^ 0x10);
    }
}
