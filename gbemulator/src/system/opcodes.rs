#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum OpCode {
    LD_r_r,
    LD_r_n,
    LD_r_HL,
    LD_HL_r,
    LD_HL_n,
    LD_A_BC,
    LD_A_DE,
    LD_BC_A,
    LD_DE_A,
    LD_A_nn,
    LD_nn_A,
    LDH_A_C,
    LDH_C_A,
    LDH_A_n,
    LDH_n_A,
    LD_A_HLm,
    LD_HLm_A,
    LD_A_HLMp,
    LD_HLp_A,
    LD_rr_nn,
}

impl OpCode {
    pub fn from_ir(ir: u8) -> Option<OpCode> {
        if ir == 0x36 {
            return Some(OpCode::LD_HL_n);
        } else if ir == 0x0A {
            return Some(OpCode::LD_A_BC);
        } else if ir == 0x1A {
            return Some(OpCode::LD_A_DE);
        } else if ir == 0x02 {
            return Some(OpCode::LD_BC_A);
        } else if ir == 0x12 {
            return Some(OpCode::LD_DE_A);
        } else if (ir >> 6) & 0x03 == 0x00 && ir & 0x07 == 0x06 {
            return Some(OpCode::LD_r_n);
        } else if (ir >> 6) & 0x03 == 0x01 {
            let target_reg = (ir >> 3) & 0x07;
            let source_reg = ir & 0x07;
            if source_reg != 0x06 && target_reg != 0x06 {
                return Some(OpCode::LD_r_r);
            } else if source_reg == 0x06 {
                return Some(OpCode::LD_r_HL);
            } else {
                return Some(OpCode::LD_HL_r);
            }
        } else if (ir >> 6) & 0x03 == 0x00 && ir & 0x0F == 0x01 {
            return Some(OpCode::LD_rr_nn);
        }

        return None;
    }
}

pub const LD_A_BC: u8 = 0x0A;
pub const LD_B_n: u8 = 0x06;
pub const LD_C_n: u8 = 0x0E;
pub const LD_B_C: u8 = 0x41;
pub const LD_HL_nn: u8 = 0x31;
pub const LD_B_HL: u8 = 0x46;
pub const LD_HL_B: u8 = 0x70;
pub const LD_HL_n: u8 = 0x36;
pub const LD_BC_nn: u8 = 0x01;
pub const LD_A_n: u8 = 0x16;
pub const LD_BC_A: u8 = 0x02;
pub const LD_DE_nn: u8 = 0x21;
pub const LD_A_DE: u8 = 0x1A;
pub const LD_DE_A: u8 = 0x12;
