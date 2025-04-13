pub struct SM83Snapshot {
    pub address_bus: u16,
    pub data_bus: u8,
    pub ir: u8,
    pub ie: u8,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
    pub ime: bool,
}

#[allow(dead_code)]
impl SM83Snapshot {
    pub fn new() -> Self {
        SM83Snapshot {
            address_bus: 0,
            data_bus: 0,
            ir: 0,
            ie: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            ime: false,
        }
    }

    pub fn with_ime(mut self, v: bool) -> Self {
        self.ime = v;
        self
    }

    pub fn with_address_bus(mut self, v: u16) -> Self {
        self.address_bus = v;
        self
    }

    pub fn with_sp(mut self, v: u16) -> Self {
        self.sp = v;
        self
    }

    pub fn with_pc(mut self, v: u16) -> Self {
        self.pc = v;
        self
    }

    pub fn with_af(mut self, v: u16) -> Self {
        self.a = ((v & 0xFF00) >> 8) as u8;
        self.f = (v & 0x00FF) as u8;
        self
    }

    pub fn with_bc(mut self, v: u16) -> Self {
        self.b = ((v & 0xFF00) >> 8) as u8;
        self.c = (v & 0x00FF) as u8;
        self
    }

    pub fn with_de(mut self, v: u16) -> Self {
        self.d = ((v & 0xFF00) >> 8) as u8;
        self.e = (v & 0x00FF) as u8;
        self
    }

    pub fn with_hl(mut self, v: u16) -> Self {
        self.h = ((v & 0xFF00) >> 8) as u8;
        self.l = (v & 0x00FF) as u8;
        self
    }

    pub fn with_data_bus(mut self, v: u8) -> Self {
        self.data_bus = v;
        self
    }

    pub fn with_ir(mut self, v: u8) -> Self {
        self.ir = v;
        self
    }

    pub fn with_ie(mut self, v: u8) -> Self {
        self.ie = v;
        self
    }

    pub fn with_a(mut self, v: u8) -> Self {
        self.a = v;
        self
    }

    pub fn with_b(mut self, v: u8) -> Self {
        self.b = v;
        self
    }

    pub fn with_c(mut self, v: u8) -> Self {
        self.c = v;
        self
    }

    pub fn with_d(mut self, v: u8) -> Self {
        self.d = v;
        self
    }

    pub fn with_e(mut self, v: u8) -> Self {
        self.e = v;
        self
    }

    pub fn with_f(mut self, v: u8) -> Self {
        self.f = v;
        self
    }

    pub fn with_h(mut self, v: u8) -> Self {
        self.h = v;
        self
    }

    pub fn with_l(mut self, v: u8) -> Self {
        self.l = v;
        self
    }

    pub fn compare(&self, other: &SM83Snapshot) -> Result<(), String> {
        let mut error = String::new();
        if self.address_bus != other.address_bus {
            error += format!(
                "address_bus is not equal: {} vs {}\n",
                self.address_bus, other.address_bus
            )
            .as_str();
        }
        if self.data_bus != other.data_bus {
            error += format!(
                "data_bus is not equal: {} vs {}\n",
                self.data_bus, other.data_bus
            )
            .as_str();
        }
        if self.ir != other.ir {
            error += format!("ir is not equal: {} vs {}\n", self.ir, other.ir).as_str();
        }
        if self.ie != other.ie {
            error += format!("ie is not equal: {} vs {}\n", self.ie, other.ie).as_str();
        }
        if self.a != other.a {
            error += format!("a is not equal: {:x} vs {:x}\n", self.a, other.a).as_str();
        }
        if self.b != other.b {
            error += format!("b is not equal: {:x} vs {:x}\n", self.b, other.b).as_str();
        }
        if self.c != other.c {
            error += format!("c is not equal: {:x} vs {:x}\n", self.c, other.c).as_str();
        }
        if self.d != other.d {
            error += format!("d is not equal: {:x} vs {:x}\n", self.d, other.d).as_str();
        }
        if self.e != other.e {
            error += format!("e is not equal: {:x} vs {:x}\n", self.e, other.e).as_str();
        }
        if self.f != other.f {
            error += format!("f is not equal: {:x} vs {:x}\n", self.f, other.f).as_str();
        }
        if self.h != other.h {
            error += format!("h is not equal: {:x} vs {:x}\n", self.h, other.h).as_str();
        }
        if self.l != other.l {
            error += format!("l is not equal: {:x} vs {:x}\n", self.l, other.l).as_str();
        }
        if self.sp != other.sp {
            error += format!("sp is not equal: {} vs {}\n", self.sp, other.sp).as_str();
        }
        if self.pc != other.pc {
            error += format!("pc is not equal: {} vs {}\n", self.pc, other.pc).as_str();
        }
        if self.ime != other.ime {
            error += format!("ime is not equal: {} vs {}\n", self.ime, other.ime).as_str();
        }
        if error.is_empty() {
            return Ok(());
        }
        return Err(error);
    }
}
