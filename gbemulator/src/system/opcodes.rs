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
pub const LD_nn_A: u8 = 0xFA;
pub const LD_A_nn: u8 = 0xEA;
pub const LDH_A_C: u8 = 0xF2;
pub const LDH_C_A: u8 = 0xE2;
pub const LDH_A_n: u8 = 0xF0;
pub const LDH_n_A: u8 = 0xE0;
pub const LD_A_HLm: u8 = 0x3A;
pub const LD_HLm_A: u8 = 0x32;
pub const LD_A_HLp: u8 = 0x2A;
pub const LD_HLp_A: u8 = 0x22;

#[derive(Debug, PartialEq)]
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
    LD_A_HLp,
    LD_HLp_A,
    LD_rr_nn,
}

impl OpCode {
    pub fn from_ir(ir: u8) -> Option<OpCode> {
        if ir == LD_HL_n {
            return Some(OpCode::LD_HL_n);
        } else if ir == LD_A_BC {
            return Some(OpCode::LD_A_BC);
        } else if ir == LD_A_DE {
            return Some(OpCode::LD_A_DE);
        } else if ir == LD_BC_A {
            return Some(OpCode::LD_BC_A);
        } else if ir == LD_DE_A {
            return Some(OpCode::LD_DE_A);
        } else if ir == LD_A_nn {
            return Some(OpCode::LD_A_nn);
        } else if ir == LD_nn_A {
            return Some(OpCode::LD_nn_A);
        } else if ir == LDH_A_C {
            return Some(OpCode::LDH_A_C);
        } else if ir == LDH_C_A {
            return Some(OpCode::LDH_C_A);
        } else if ir == LDH_A_n {
            return Some(OpCode::LDH_A_n);
        } else if ir == LDH_n_A {
            return Some(OpCode::LDH_n_A);
        } else if ir == LD_A_HLm {
            return Some(OpCode::LD_A_HLm);
        } else if ir == LD_HLm_A {
            return Some(OpCode::LD_HLm_A);
        } else if ir == LD_A_HLp {
            return Some(OpCode::LD_A_HLp);
        } else if ir == LD_HLp_A {
            return Some(OpCode::LD_HLp_A);
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
