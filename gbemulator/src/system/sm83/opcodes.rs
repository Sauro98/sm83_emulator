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
// 16bit arithmetic instructions
pub const INC_rr_base: u8 = 0x03;
pub const INC_rr_mask: u8 = 0b1100_1111;
pub const INC_BC: u8 = 0x03;
pub const DEC_rr_base: u8 = 0x0B;
pub const DEC_rr_mask: u8 = 0b1100_1111;
pub const DEC_BC: u8 = 0x0B;
pub const ADD_HL_rr_base: u8 = 0x09;
pub const ADD_HL_rr_mask: u8 = 0b1100_1111;
pub const ADD_HL_BC: u8 = 0x09;
pub const ADD_SP_e: u8 = 0xE8;
// rotate shift and bit operations
pub const RLCA: u8 = 0x07;
pub const RRCA: u8 = 0x0F;
pub const RLA: u8 = 0x17;
pub const RRA: u8 = 0x1F;
pub const CB_PREFIX: u8 = 0xCB;
// control flow instructions
pub const JP_NN: u8 = 0xC3;
pub const JP_HL: u8 = 0xE9;
pub const JP_CC_NN_BASE: u8 = 0xC2;
pub const JP_CC_NN_MASK: u8 = 0b1110_0111;
pub const JP_NZ_NN: u8 = 0xC2;
pub const JR_E: u8 = 0x18;
pub const JR_CC_E_BASE: u8 = 0x20;
pub const JR_CC_E_MASK: u8 = 0b1110_0111;
pub const JR_NZ_E: u8 = 0x20;
pub const CALL_NN: u8 = 0xCD;
pub const CALL_CC_NN_BASE: u8 = 0xC4;
pub const CALL_CC_NN_MASK: u8 = 0b1110_0111;
pub const CALL_NZ_NN: u8 = 0xC4;
pub const RET: u8 = 0xC9;
pub const RET_CC_BASE: u8 = 0xC0;
pub const RET_CC_MASK: u8 = 0b1110_0111;
pub const RET_NZ: u8 = 0xC0;
pub const RETI: u8 = 0xD9;
pub const RST_N_BASE: u8 = 0xC7;
pub const RST_N_MASK: u8 = 0b1100_0111;
pub const RST_18: u8 = 0xDF;
pub const DI: u8 = 0xF3;
pub const EI: u8 = 0xFB;
pub const HALT: u8 = 0x76;
pub const STOP: u8 = 0x10;

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
    INC_rr,
    DEC_rr,
    ADD_HL_rr,
    ADD_SP_e,
    RLCA,
    RRCA,
    RLA,
    RRA,
    CB_PREFIX,
    JP_NN,
    JP_HL,
    JP_CC_NN,
    JR_E,
    JR_CC_E,
    CALL_NN,
    CALL_CC_NN,
    RET,
    RET_CC,
    RETI,
    RST_N,
    DI,
    EI,
    HALT,
    STOP,
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
        } else if ir == ADD_SP_e {
            return Some(OpCode::ADD_SP_e);
        } else if ir == RLCA {
            return Some(OpCode::RLCA);
        } else if ir == RRCA {
            return Some(OpCode::RRCA);
        } else if ir == RLA {
            return Some(OpCode::RLA);
        } else if ir == RRA {
            return Some(OpCode::RRA);
        } else if ir == CB_PREFIX {
            return Some(OpCode::CB_PREFIX);
        } else if ir == JP_NN {
            return Some(OpCode::JP_NN);
        } else if ir == JP_HL {
            return Some(OpCode::JP_HL);
        } else if ir == JR_E {
            return Some(OpCode::JR_E);
        } else if ir == CALL_NN {
            return Some(OpCode::CALL_NN);
        } else if ir == RET {
            return Some(OpCode::RET);
        } else if ir == RETI {
            return Some(OpCode::RETI);
        } else if ir == DI {
            return Some(OpCode::DI);
        } else if ir == EI {
            return Some(OpCode::EI);
        } else if ir == HALT {
            return Some(OpCode::HALT);
        } else if ir == STOP {
            return Some(OpCode::STOP);
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
        } else if ir & INC_rr_mask == INC_rr_base {
            return Some(OpCode::INC_rr);
        } else if ir & DEC_rr_mask == DEC_rr_base {
            return Some(OpCode::DEC_rr);
        } else if ir & ADD_HL_rr_mask == ADD_HL_rr_base {
            return Some(OpCode::ADD_HL_rr);
        } else if ir & JP_CC_NN_MASK == JP_CC_NN_BASE {
            return Some(OpCode::JP_CC_NN);
        } else if ir & JR_CC_E_MASK == JR_CC_E_BASE {
            return Some(OpCode::JR_CC_E);
        } else if ir & CALL_CC_NN_MASK == CALL_CC_NN_BASE {
            return Some(OpCode::CALL_CC_NN);
        } else if ir & RET_CC_MASK == RET_CC_BASE {
            return Some(OpCode::RET_CC);
        } else if ir & RST_N_MASK == RST_N_BASE {
            return Some(OpCode::RST_N);
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

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CBPrefixOpCode {
    RLC_r,
    RLC_HL,
    RRC_r,
    RRC_HL,
    RL_r,
    RL_HL,
    RR_r,
    RR_HL,
    SLA_r,
    SLA_HL,
    SRA_r,
    SRA_HL,
    SWAP_r,
    SWAP_HL,
    SRL_r,
    SRL_HL,
    BIT_b_r,
    BIT_b_HL,
    RES_b_r,
    RES_b_HL,
    SET_b_r,
    SET_b_HL,
}

pub const RLC_R_BASE: u8 = 0x00;
pub const RLC_R_MASK: u8 = 0b1111_1000;
pub const RLC_B: u8 = 0x00;
pub const RLC_HL: u8 = 0x06;
pub const RRC_R_BASE: u8 = 0x08;
pub const RRC_R_MASK: u8 = 0b1111_1000;
pub const RRC_B: u8 = 0x08;
pub const RRC_HL: u8 = 0x0E;
pub const RL_R_BASE: u8 = 0x10;
pub const RL_R_MASK: u8 = 0b1111_1000;
pub const RL_B: u8 = 0x10;
pub const RL_HL: u8 = 0x16;
pub const RR_R_BASE: u8 = 0x18;
pub const RR_R_MASK: u8 = 0b1111_1000;
pub const RR_B: u8 = 0x18;
pub const RR_HL: u8 = 0x1E;
pub const SLA_R_BASE: u8 = 0x20;
pub const SLA_R_MASK: u8 = 0b1111_1000;
pub const SLA_B: u8 = 0x20;
pub const SLA_HL: u8 = 0x26;
pub const SRA_R_BASE: u8 = 0x28;
pub const SRA_R_MASK: u8 = 0b1111_1000;
pub const SRA_B: u8 = 0x28;
pub const SRA_HL: u8 = 0x2E;
pub const SWAP_R_BASE: u8 = 0x30;
pub const SWAP_R_MASK: u8 = 0b1111_1000;
pub const SWAP_B: u8 = 0x30;
pub const SWAP_HL: u8 = 0x36;
pub const SRL_R_BASE: u8 = 0x38;
pub const SRL_R_MASK: u8 = 0b1111_1000;
pub const SRL_B: u8 = 0x38;
pub const SRL_HL: u8 = 0x3E;
pub const BIT_B_R_BASE: u8 = 0x40;
pub const BIT_B_R_MASK: u8 = 0b1100_0000;
pub const BIT_0_B: u8 = 0x40;
pub const BIT_B_HL_BASE: u8 = 0x46;
pub const BIT_B_HL_MASK: u8 = 0b11000111;
pub const BIT_0_HL: u8 = 0x46;
pub const RES_B_R_BASE: u8 = 0x80;
pub const RES_B_R_MASK: u8 = 0b1100_0000;
pub const RES_0_B: u8 = 0x80;
pub const RES_B_HL_BASE: u8 = 0x86;
pub const RES_B_HL_MASK: u8 = 0b11000111;
pub const RES_0_HL: u8 = 0x86;
pub const SET_B_R_BASE: u8 = 0xC0;
pub const SET_B_R_MASK: u8 = 0b1100_0000;
pub const SET_0_B: u8 = 0xC0;
pub const SET_B_HL_BASE: u8 = 0xC6;
pub const SET_B_HL_MASK: u8 = 0b11000111;
pub const SET_0_HL: u8 = 0xC6;

impl CBPrefixOpCode {
    pub fn from_ir(ir: u8) -> Option<CBPrefixOpCode> {
        if ir == RLC_HL {
            return Some(Self::RLC_HL);
        } else if ir == RRC_HL {
            return Some(Self::RRC_HL);
        } else if ir == RL_HL {
            return Some(Self::RL_HL);
        } else if ir == RR_HL {
            return Some(Self::RR_HL);
        } else if ir == SLA_HL {
            return Some(Self::SLA_HL);
        } else if ir == SRA_HL {
            return Some(Self::SRA_HL);
        } else if ir == SWAP_HL {
            return Some(Self::SWAP_HL);
        } else if ir == SRL_HL {
            return Some(Self::SRL_HL);
        } else if ir & RLC_R_MASK == RLC_R_BASE {
            return Some(Self::RLC_r);
        } else if ir & RRC_R_MASK == RRC_R_BASE {
            return Some(Self::RRC_r);
        } else if ir & RL_R_MASK == RL_R_BASE {
            return Some(Self::RL_r);
        } else if ir & RR_R_MASK == RR_R_BASE {
            return Some(Self::RR_r);
        } else if ir & SLA_R_MASK == SLA_R_BASE {
            return Some(Self::SLA_r);
        } else if ir & SRA_R_MASK == SRA_R_BASE {
            return Some(Self::SRA_r);
        } else if ir & SWAP_R_MASK == SWAP_R_BASE {
            return Some(Self::SWAP_r);
        } else if ir & SRL_R_MASK == SRL_R_BASE {
            return Some(Self::SRL_r);
        } else if ir & BIT_B_R_MASK == BIT_B_R_BASE {
            if ir & BIT_B_HL_MASK == BIT_B_HL_BASE {
                return Some(Self::BIT_b_HL);
            } else {
                return Some(Self::BIT_b_r);
            };
        } else if ir & SET_B_R_MASK == SET_B_R_BASE {
            if ir & SET_B_HL_MASK == SET_B_HL_BASE {
                return Some(Self::SET_b_HL);
            } else {
                return Some(Self::SET_b_r);
            }
        } else if ir & RES_B_R_MASK == RES_B_R_BASE {
            if ir & RES_B_HL_MASK == RES_B_HL_BASE {
                return Some(Self::RES_b_HL);
            } else {
                return Some(Self::RES_b_r);
            }
        }
        return None;
    }
}
