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
}
