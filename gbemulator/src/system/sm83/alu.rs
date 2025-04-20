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
        if sum > 0xFF {
            flags |= 0x10;
        }
        if half_sum > 0x0F {
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
        let (diff, _) = Self::add3(v1, Self::twos_complement(v2), Self::twos_complement(v3));
        let mut flags = 0x40;
        if (v1 as u16) < (v2 as u16 + v3 as u16) {
            flags |= 0x10;
        }
        if ((v1 ^ v2 ^ v3 ^ diff) & 0x10) != 0 {
            flags |= 0x20;
        }
        if diff == 0 {
            flags |= 0x80;
        }
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

    pub fn decimal_adjust(v: u8, flags: u8) -> (u8, u8) {
        let neg = (flags & 0x40) > 0;
        let half_carry = (flags & 0x20) > 0;
        let carry = (flags & 0x10) > 0;
        let mut adj = if half_carry { 0x06 } else { 0x00 } | if carry { 0x60 } else { 0x00 };

        if !neg {
            if (v & 0x0F) > 0x09 {
                adj |= 0x06;
            }
            if v > 0x99 {
                adj |= 0x60;
            }
        }
        let (res, mut flags) = if neg {
            Self::sub(v, adj)
        } else {
            Self::add(v, adj)
        };
        if adj >= 0x60 {
            flags |= 0x10;
        }
        flags &= 0b1101_1111; // unset half carry flag
        (res, flags)
    }

    pub fn rotate_left(v: u8, carry: u8) -> (u8, u8) {
        let mut res = ((v & 0x7F) << 1) & 0xFE; // remove last bit, rotate left and clear last bit
        res = res | carry; // add prev carry bit if it was one
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x80 > 0 {
            // if last bit set, set carry
            flags |= 0x10;
        }
        (res, flags)
    }

    pub fn shift_left_arithmetic(v: u8) -> (u8, u8) {
        let res = ((v & 0x7F) << 1) & 0xFE; // remove last bit, rotate left and clear last bit
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x80 > 0 {
            // if last bit set, set carry
            flags |= 0x10;
        }
        (res, flags)
    }

    pub fn rotate_left_circular(v: u8) -> (u8, u8) {
        let mut res = ((v & 0x7F) << 1) & 0xFE; // remove last bit, rotate left and clear last bit
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x80 > 0 {
            // if last bit set, set both carry and first bit
            flags = 0x10;
            res |= 0x01;
        }
        (res, flags)
    }

    pub fn rotate_right(v: u8, carry: u8) -> (u8, u8) {
        let mut res = ((v & 0xFE) >> 1) & 0x7F; // remove first bit, rotate right and clear first bit
        res = res | (carry << 7); // add prev carry bit if it was one
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x01 > 0 {
            // if first bit set, set carry
            flags |= 0x10;
        }

        (res, flags)
    }

    pub fn shift_right_arithmetic(v: u8) -> (u8, u8) {
        let mut res = ((v & 0xFE) >> 1) & 0x7F; // remove first bit, rotate right and clear first bit
        res = res | (v & 0x80); // add last bit again
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x01 > 0 {
            // if first bit set, set carry
            flags |= 0x10;
        }
        (res, flags)
    }

    pub fn shift_right_logical(v: u8) -> (u8, u8) {
        let res = ((v & 0xFE) >> 1) & 0x7F; // remove first bit, rotate right and clear first bit
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x01 > 0 {
            // if first bit set, set carry
            flags |= 0x10;
        }
        (res, flags)
    }

    pub fn rotate_right_circular(v: u8) -> (u8, u8) {
        let mut res = ((v & 0xFE) >> 1) & 0x7F; // remove first bit, rotate right and clear first bit
        let mut flags = if res == 0 { 0x80 } else { 0 };
        if v & 0x01 > 0 {
            // if first bit set, set both carry and last bit
            flags = 0x10;
            res |= 0x80;
        }
        (res, flags)
    }

    pub fn swap_nibbles(v: u8) -> (u8, u8) {
        let res = ((v & 0x0F) << 4) | ((v & 0xF0) >> 4);
        let flags = if res == 0 { 0x80 } else { 0 };
        return (res, flags);
    }

    pub fn test_bit(v: u8, bit: u8) -> u8 {
        let bit = (v >> bit) & 0x01;
        if bit > 0 {
            0x20 // H
        } else {
            0xA0 // Z,H
        }
    }

    pub fn set_bit(v: u8, bit: u8) -> u8 {
        let bit = 1 << bit;
        v | bit
    }

    pub fn reset_bit(v: u8, bit: u8) -> u8 {
        let bit = !(1 << bit);
        v & bit
    }

    pub fn add_16_signed(val: u16, offset: u8) -> u16 {
        let lsb = (val & 0x00FF) as u8;
        let msb = ((val & 0xFF00) >> 8) as u8;
        let (lsb, flags) = ALU::add(lsb, offset);
        let (msb, _) = if (flags & 0x10) > 0 && (offset & 0x80) == 0 {
            ALU::add(msb, 1)
        } else if (flags & 0x10) == 0 && (offset & 0x80) > 0 {
            ALU::sub(msb, 1)
        } else {
            (msb, 0)
        };
        ((msb as u16) << 8) | (lsb as u16)
    }
}
