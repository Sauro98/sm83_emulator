// 8 bit load operations
pub const LD_A_BC: u8 = 0x0A;
pub const LD_B_n: u8 = 0x06;
pub const LD_C_n: u8 = 0x0E;
pub const LD_B_C: u8 = 0x41;
pub const LD_B_HL: u8 = 0x46;
pub const LD_HL_B: u8 = 0x70;
pub const LD_HL_n: u8 = 0x36;
pub const LD_A_n: u8 = 0x16;
pub const LD_BC_A: u8 = 0x02;
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
// 16 bit load operations
pub const LD_DE_nn: u8 = 0x21;
pub const LD_BC_nn: u8 = 0x01;
pub const LD_HL_nn: u8 = 0x31;
pub const LD_nn_SP: u8 = 0x08;
pub const LD_SP_HL: u8 = 0xF9;
pub const PUSH_rr_base: u8 = 0xC5;
pub const PUSH_rr_mask: u8 = 0xCF;
pub const PUSH_BC: u8 = 0xC5;
pub const PUSH_HL: u8 = 0xF5;
pub const POP_rr_base: u8 = 0xC1;
pub const POP_rr_mask: u8 = PUSH_rr_mask;
pub const POP_BC: u8 = 0xC1;
pub const POP_HL: u8 = 0xF1;
pub const LD_HL_SPe: u8 = 0xF8;
// 8 bit arithmetic and logical instructions
pub const ADD_r_base: u8 = 0x80;
pub const ADD_r_mask: u8 = 0xF8;
pub const ADD_B: u8 = 0x80;
pub const ADD_C: u8 = 0x81;
pub const ADD_HL: u8 = 0x86;
pub const ADD_n: u8 = 0xC6;
pub const ADC_r_base: u8 = 0x88;
pub const ADC_B: u8 = 0x88;
pub const ADC_C: u8 = 0x89;
pub const ADC_HL: u8 = 0x8E;
pub const ADC_n: u8 = 0xCE;
pub const SUB_r_base: u8 = 0x90;
pub const SUB_r_mask: u8 = ADD_r_mask;
pub const SUB_B: u8 = 0x90;
pub const SUB_HL: u8 = 0x96;
pub const SUB_n: u8 = 0xD6;
pub const SBC_r_base: u8 = 0x98;
pub const SBC_B: u8 = 0x98;
pub const SBC_HL: u8 = 0x9E;
pub const SBC_n: u8 = 0xDE;
pub const CP_r_base: u8 = 0xB8;
pub const CP_r_mask: u8 = ADD_r_mask;
pub const CP_B: u8 = 0xB8;
pub const CP_HL: u8 = 0xBE;
pub const CP_n: u8 = 0xFE;
pub const INC_r_base: u8 = 0x04;
pub const INC_r_mask: u8 = 0b11000111;
pub const INC_B: u8 = 0x04;
pub const INC_HL: u8 = 0x34;
pub const DEC_r_base: u8 = 0x05;
pub const DEC_B: u8 = 0x05;
pub const DEC_r_mask: u8 = INC_r_mask;
pub const DEC_HL: u8 = 0x35;
pub const AND_r_base: u8 = 0xA0;
pub const AND_r_mask: u8 = ADD_r_mask;
pub const AND_B: u8 = 0xA0;
pub const AND_HL: u8 = 0xA6;
pub const AND_n: u8 = 0xE6;
pub const OR_r_base: u8 = 0xB0;
pub const OR_r_mask: u8 = ADD_r_mask;
pub const OR_B: u8 = 0xB0;
pub const OR_HL: u8 = 0xB6;
pub const OR_n: u8 = 0xF6;
pub const XOR_r_base: u8 = 0xA8;
pub const XOR_r_mask: u8 = ADD_r_mask;
pub const XOR_B: u8 = 0xA8;
pub const XOR_HL: u8 = 0xAE;
pub const XOR_n: u8 = 0xEE;
pub const CCF: u8 = 0x3F;
pub const SCF: u8 = 0x37;
pub const DAA: u8 = 0x27;
pub const CPL: u8 = 0x2F;

// misc
pub const NOP: u8 = 0x00;

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
    LD_nn_SP,
    LD_SP_HL,
    PUSH_rr,
    POP_rr,
    LD_HL_SPe,
    ADD_r,
    ADC_r,
    ADD_HL,
    ADD_n,
    ADC_HL,
    ADC_n,
    SUB_r,
    SUB_HL,
    SUB_n,
    SBC_r,
    SBC_HL,
    SBC_n,
    CP_r,
    CP_HL,
    CP_n,
    INC_r,
    INC_HL,
    DEC_r,
    DEC_HL,
    AND_r,
    AND_HL,
    AND_n,
    OR_r,
    OR_HL,
    OR_n,
    XOR_r,
    XOR_HL,
    XOR_n,
    NOP,
    CCF,
    SCF,
    DAA,
    CPL,
}

impl OpCode {
    pub fn from_ir(ir: u8) -> Option<OpCode> {
        if ir == LD_HL_n {
            return Some(OpCode::LD_HL_n);
        } else if ir == LD_A_BC {
            return Some(OpCode::LD_A_BC);
        } else if ir == NOP {
            return Some(OpCode::NOP);
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
        } else if ir == LD_nn_SP {
            return Some(OpCode::LD_nn_SP);
        } else if ir == LD_SP_HL {
            return Some(OpCode::LD_SP_HL);
        } else if ir == LD_HL_SPe {
            return Some(OpCode::LD_HL_SPe);
        } else if ir == ADD_HL {
            return Some(OpCode::ADD_HL);
        } else if ir == ADD_n {
            return Some(OpCode::ADD_n);
        } else if ir == ADC_HL {
            return Some(OpCode::ADC_HL);
        } else if ir == ADC_n {
            return Some(OpCode::ADC_n);
        } else if ir == SUB_HL {
            return Some(OpCode::SUB_HL);
        } else if ir == SUB_n {
            return Some(OpCode::SUB_n);
        } else if ir == SBC_HL {
            return Some(OpCode::SBC_HL);
        } else if ir == SBC_n {
            return Some(OpCode::SBC_n);
        } else if ir == CP_HL {
            return Some(OpCode::CP_HL);
        } else if ir == CP_n {
            return Some(OpCode::CP_n);
        } else if ir == INC_HL {
            return Some(OpCode::INC_HL);
        } else if ir == DEC_HL {
            return Some(OpCode::DEC_HL);
        } else if ir == AND_HL {
            return Some(OpCode::AND_HL);
        } else if ir == OR_HL {
            return Some(OpCode::OR_HL);
        } else if ir == XOR_HL {
            return Some(OpCode::XOR_HL);
        } else if ir == AND_n {
            return Some(OpCode::AND_n);
        } else if ir == OR_n {
            return Some(OpCode::OR_n);
        } else if ir == XOR_n {
            return Some(OpCode::XOR_n);
        } else if ir == CCF {
            return Some(OpCode::CCF);
        } else if ir == SCF {
            return Some(OpCode::SCF);
        } else if ir == DAA {
            return Some(OpCode::DAA);
        } else if ir == CPL {
            return Some(OpCode::CPL);
        } else if ir & PUSH_rr_mask == PUSH_rr_base {
            return Some(OpCode::PUSH_rr);
        } else if ir & POP_rr_mask == POP_rr_base {
            return Some(OpCode::POP_rr);
        } else if ir & ADD_r_mask == ADD_r_base {
            return Some(OpCode::ADD_r);
        } else if ir & ADD_r_mask == ADC_r_base {
            return Some(OpCode::ADC_r);
        } else if ir & SUB_r_mask == SUB_r_base {
            return Some(OpCode::SUB_r);
        } else if ir & SUB_r_mask == SBC_r_base {
            return Some(OpCode::SBC_r);
        } else if ir & CP_r_mask == CP_r_base {
            return Some(OpCode::CP_r);
        } else if ir & INC_r_mask == INC_r_base {
            return Some(OpCode::INC_r);
        } else if ir & DEC_r_mask == DEC_r_base {
            return Some(OpCode::DEC_r);
        } else if ir & AND_r_mask == AND_r_base {
            return Some(OpCode::AND_r);
        } else if ir & OR_r_mask == OR_r_base {
            return Some(OpCode::OR_r);
        } else if ir & XOR_r_mask == XOR_r_base {
            return Some(OpCode::XOR_r);
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
