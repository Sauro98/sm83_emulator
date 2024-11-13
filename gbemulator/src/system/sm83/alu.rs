pub struct ALU {}

impl ALU {
    pub fn twos_complement(v: u8) -> u8 {
        let v_inv = !v;
        let res = (v_inv as u16) + 1;
        (res & 0x00FF) as u8
    }

    pub fn add3(v1: u8, v2: u8, v3: u8) -> (u8, u8) {
        let sum = (v1 as u16) + (v2 as u16) + (v3 as u16);
        let half_sum = (v1 & 0x0F) + (v2 & 0x0F) + (v3 & 0x0F);
        let sum_u8 = (sum & 0x00FF) as u8;
        let mut flags = 0;
        if sum & 0x0100 != 0 {
            flags |= 0x10;
        }
        if half_sum & 0x10 != 0 {
            flags |= 0x20;
        }
        if sum_u8 == 0 {
            flags |= 0x80;
        }
        (sum_u8, flags)
    }
    pub fn add(v1: u8, v2: u8) -> (u8, u8) {
        Self::add3(v1, v2, 0)
    }

    pub fn sub3(v1: u8, v2: u8, v3: u8) -> (u8, u8) {
        let (diff, mut flags) =
            Self::add3(v1, Self::twos_complement(v2), Self::twos_complement(v3));
        flags |= 0x40;
        (diff, flags)
    }
    pub fn sub(v1: u8, v2: u8) -> (u8, u8) {
        Self::sub3(v1, v2, 0)
    }

    pub fn increment(v: u8) -> (u8, u8) {
        let (res, flags) = Self::add(v, 1);
        (res, flags & 0b1110_0000)
    }

    pub fn decrement(v: u8) -> (u8, u8) {
        let (res, flags) = Self::sub(v, 1);
        (res, flags & 0b1110_0000)
    }

    pub fn and(v1: u8, v2: u8) -> (u8, u8) {
        let res = v1 & v2;
        let mut flags = 0x20;
        if res == 0 {
            flags |= 0x80;
        }
        (res, flags)
    }

    pub fn or(v1: u8, v2: u8) -> (u8, u8) {
        let res = v1 | v2;
        let mut flags = 0x00;
        if res == 0 {
            flags |= 0x80;
        }
        (res, flags)
    }

    pub fn xor(v1: u8, v2: u8) -> (u8, u8) {
        let res = v1 ^ v2;
        let mut flags = 0x00;
        if res == 0 {
            flags |= 0x80;
        }
        (res, flags)
    }

    pub fn decimal_adjust(v: u8, carry: bool, half_carry: bool) -> (u8, u8) {
        let upper_nibble = (v & 0xF0) >> 4;
        let lower_nibble = v & 0x0F;
        let mut adj = 0x00;
        if lower_nibble > 9 || half_carry {
            adj |= 0x06;
        }
        if upper_nibble > 9 || carry {
            adj |= 0x60;
        }
        let (res, mut flags) = Self::add(v, adj);
        flags &= 0b1101_1111; // unset half carry flag
        (res, flags)
    }
}
