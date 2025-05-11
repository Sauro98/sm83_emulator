pub mod alu;
pub mod idu;
pub mod opcodes;
pub mod registers;
pub mod snapshot;

use crate::system::clock::SystemClock;
use crate::system::ram::RAM;
use alu::ALU;
use idu::IDU;
use opcodes::{CBPrefixOpCode, OpCode};
use registers::{RegisterFile, RegisterName};
use snapshot::SM83Snapshot;

pub struct SM83 {
    internal_clock: SystemClock,
    last_execution_time: std::time::Instant,
    iteration_time: u128,
    pub cycle_count: u128,
    register_file: RegisterFile,
    address_bus: u16,
    data_bus: u8,
    ime: bool,
}

impl SM83 {
    pub fn new(clock_frequency: f32) -> SM83 {
        SM83 {
            internal_clock: SystemClock::from_frequency(clock_frequency),
            last_execution_time: std::time::Instant::now(),
            iteration_time: 1,
            cycle_count: 0,
            register_file: RegisterFile::new(),
            address_bus: 0,
            data_bus: 0,
            ime: false,
        }
    }

    #[allow(dead_code)]
    pub fn load_snapshot(&mut self, snapshot: SM83Snapshot) {
        self.address_bus = snapshot.address_bus;
        self.data_bus = snapshot.data_bus;
        self.register_file.set_ir(snapshot.ir);
        self.register_file.set_ie(snapshot.ie);
        self.ime = snapshot.ime;
        self.register_file.set_a(snapshot.a);
        self.register_file.set_b(snapshot.b);
        self.register_file.set_c(snapshot.c);
        self.register_file.set_d(snapshot.d);
        self.register_file.set_e(snapshot.e);
        self.register_file.set_f(snapshot.f);
        self.register_file.set_h(snapshot.h);
        self.register_file.set_l(snapshot.l);
        self.register_file.set_sp(snapshot.sp);
        self.register_file.set_pc(snapshot.pc);
    }

    pub fn to_snapshot(&self) -> SM83Snapshot {
        let rf = &self.register_file;
        let snapshot = SM83Snapshot::new()
            .with_ir(rf.get_ir())
            .with_ie(rf.get_ie())
            .with_ime(self.ime)
            .with_a(rf.get_a())
            .with_b(rf.get_b())
            .with_c(rf.get_c())
            .with_d(rf.get_d())
            .with_e(rf.get_e())
            .with_f(rf.get_f())
            .with_h(rf.get_h())
            .with_l(rf.get_l())
            .with_sp(rf.get_sp())
            .with_pc(rf.get_pc());
        return snapshot;
    }

    pub fn interrupt_enabled(&self) -> bool {
        self.ime
    }

    pub fn reset(&mut self, ram: &RAM) {
        self.register_file.set_pc(0x00);
        self.fetch_cycle(ram);
    }

    fn idu_increment(&mut self) {
        let res = IDU::increment(self.address_bus);
        self.address_bus = res;
    }

    fn idu_decrement(&mut self) {
        let res = IDU::decrement(self.address_bus);
        self.address_bus = res;
    }

    fn increase_pc(&mut self) {
        self.address_bus = self.register_file.get_pc();
        self.idu_increment();
        self.register_file.set_pc(self.address_bus);
    }

    pub fn fetch_cycle(&mut self, ram: &RAM) {
        self.address_bus = self.register_file.get_pc();
        self.read_ram(ram);
        self.register_file.set_ir(self.data_bus);
        self.increase_pc();
    }

    pub fn prefetch(&mut self, ram: &RAM) {
        self.address_bus = self.register_file.get_pc();
        self.read_ram(ram);
        self.register_file.set_ir(self.data_bus);
    }

    fn read_ram(&mut self, ram: &RAM) {
        self.data_bus = ram.get_at(self.address_bus).unwrap();
    }

    fn write_ram(&self, ram: &mut RAM) {
        match ram.set_at(self.address_bus, self.data_bus) {
            Some(_) => (),
            None => panic!(
                "Failed to write {:x} to address {:x}",
                self.data_bus, self.address_bus
            ),
        }
    }

    fn push_stack(&mut self) {
        self.address_bus = self.register_file.get_sp();
        self.idu_decrement();
        self.register_file.set_sp(self.address_bus);
    }

    fn pop_stack(&mut self) {
        self.address_bus = self.register_file.get_sp();
        self.idu_increment();
        self.register_file.set_sp(self.address_bus);
    }

    fn add(&mut self, val: u8, carry: bool) {
        let (sum, flags) = if !carry {
            ALU::add(self.register_file.get_a(), val)
        } else {
            ALU::add3(
                self.register_file.get_a(),
                val,
                self.register_file.get_carry_flag(),
            )
        };
        self.register_file.set_a(sum);
        self.register_file.set_f(flags);
    }

    fn compare(&mut self, val: u8, carry: bool) -> u8 {
        let (diff, flags) = if !carry {
            ALU::sub(self.register_file.get_a(), val)
        } else {
            ALU::sub3(
                self.register_file.get_a(),
                val,
                self.register_file.get_carry_flag(),
            )
        };
        self.register_file.set_f(flags);
        diff
    }

    fn sub(&mut self, val: u8, carry: bool) {
        let diff = self.compare(val, carry);
        self.register_file.set_a(diff);
    }

    fn and(&mut self, val: u8) {
        let (res, flags) = ALU::and(self.register_file.get_a(), val);
        self.register_file.set_a(res);
        self.register_file.set_f(flags);
    }

    fn or(&mut self, val: u8) {
        let (res, flags) = ALU::or(self.register_file.get_a(), val);
        self.register_file.set_a(res);
        self.register_file.set_f(flags);
    }

    fn xor(&mut self, val: u8) {
        let (res, flags) = ALU::xor(self.register_file.get_a(), val);
        self.register_file.set_a(res);
        self.register_file.set_f(flags);
    }

    fn code_to_condition(&self, code: u8) -> bool {
        match code {
            0b00 => self.register_file.get_zero_flag() == 0,
            0b01 => self.register_file.get_zero_flag() > 0,
            0b10 => self.register_file.get_carry_flag() == 0,
            0b11 => self.register_file.get_carry_flag() > 0,
            _ => false,
        }
    }

    fn code_to_page_memory(&self, code: u8) -> u8 {
        match code {
            0b000 => 0x00,
            0b001 => 0x08,
            0b010 => 0x10,
            0b011 => 0x18,
            0b100 => 0x20,
            0b101 => 0x28,
            0b110 => 0x30,
            0b111 => 0x38,
            _ => panic!("unrecognized page address {:b}", code),
        }
    }

    fn read_16b_ram(&mut self, ram: &RAM) -> u16 {
        self.read_ram(ram);
        let value = self.data_bus as u16;
        self.increase_pc();
        self.tick_clock();
        self.read_ram(ram);
        let value = ((self.data_bus as u16) << 8) | value;
        self.increase_pc();
        self.tick_clock();
        return value;
    }

    fn tick_clock(&mut self) {
        self.internal_clock.next();
        let duration = std::time::Instant::now().duration_since(self.last_execution_time);
        if self.cycle_count == 0 {
            self.iteration_time = duration.as_nanos();
        } else {
            self.iteration_time = (self.iteration_time + duration.as_nanos()) / 2;
        }
        self.cycle_count += 1;
        self.last_execution_time = std::time::Instant::now();
    }

    pub fn next(&mut self, ram: &mut RAM) {
        let ir = self.register_file.get_ir();
        let op_code = OpCode::from_ir(ir);
        /*println!(
            "pc: 0x{:X} ir: 0x{:X}, op: {:?}",
            self.register_file.get_pc(),
            ir,
            op_code
        );*/

        match op_code {
            None => {
                panic!("Unrecognized OP CODE {:x}", ir);
            }
            Some(OpCode::LD_HL_n) => {
                // read value from ram
                self.read_ram(ram);
                self.increase_pc();
                self.tick_clock();
                //  write value to ram
                self.address_bus = self.register_file.get_hl();
                self.write_ram(ram);
                self.tick_clock();
                // fetch cycle
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_A_BC) => {
                // read value from ram
                self.address_bus = self.register_file.get_bc();
                self.read_ram(ram);
                self.tick_clock();
                // fetch cycle
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_A_DE) => {
                // read value from ram
                self.address_bus = self.register_file.get_de();
                self.read_ram(ram);
                self.tick_clock();
                // fetch cycle
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_BC_A) => {
                self.address_bus = self.register_file.get_bc();
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_DE_A) => {
                self.address_bus = self.register_file.get_de();
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_A_nn) => {
                // load lsb
                let val = self.read_16b_ram(ram);
                // read ram
                self.address_bus = val;
                self.read_ram(ram);
                self.tick_clock();
                // fetch cycle
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_nn_A) => {
                // load 16 bit value
                let val = self.read_16b_ram(ram);
                // write ram
                self.address_bus = val;
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LDH_A_C) => {
                self.address_bus = 0xFF00 | (self.register_file.get_c() as u16);
                self.read_ram(ram);
                self.tick_clock();
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LDH_C_A) => {
                self.address_bus = 0xFF00 | (self.register_file.get_c() as u16);
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LDH_A_n) => {
                self.read_ram(ram);
                self.increase_pc();
                self.tick_clock();
                self.address_bus = 0xFF00 | (self.data_bus as u16);
                self.read_ram(ram);
                self.tick_clock();
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LDH_n_A) => {
                self.read_ram(ram);
                self.increase_pc();
                self.tick_clock();
                self.address_bus = 0xFF00 | (self.data_bus as u16);
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_A_HLm) | Some(OpCode::LD_A_HLp) => {
                // read value from ram
                self.address_bus = self.register_file.get_hl();
                self.read_ram(ram);
                if op_code.unwrap() == OpCode::LD_A_HLm {
                    self.idu_decrement();
                } else {
                    self.idu_increment();
                }
                self.register_file.set_hl(self.address_bus);
                self.tick_clock();
                // fetch cycle
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_HLm_A) | Some(OpCode::LD_HLp_A) => {
                self.address_bus = self.register_file.get_hl();
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                if op_code.unwrap() == OpCode::LD_HLm_A {
                    self.idu_decrement();
                } else {
                    self.idu_increment();
                }
                self.register_file.set_hl(self.address_bus);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_r_n) => {
                // data read cycle
                let reg = (ir >> 3) & 0x07;
                self.read_ram(ram);
                self.increase_pc();
                self.tick_clock();
                // fetch cycle
                self.register_file.set(reg, self.data_bus).unwrap();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_rr_nn) => {
                let reg = ir >> 4 & 0x03;
                let value = self.read_16b_ram(ram);
                self.register_file.set16_dd(reg, value).unwrap();
                //println!("writing {:X} to register {}", value, reg);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_nn_SP) => {
                let address = self.read_16b_ram(ram);
                let value = self.register_file.get_sp();
                self.address_bus = address;
                self.data_bus = (value & 0x00FF) as u8;
                self.write_ram(ram);
                self.tick_clock();
                self.idu_increment();
                self.data_bus = ((value & 0xFF00) >> 8) as u8;
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_SP_HL) => {
                self.address_bus = self.register_file.get_hl();
                self.register_file.set_sp(self.address_bus);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::PUSH_rr) => {
                let reg = (ir & 0x30) >> 4;
                let val = self.register_file.get16_qq(reg).unwrap();
                self.push_stack();
                self.tick_clock();
                self.data_bus = ((val & 0xFF00) >> 8) as u8;
                self.write_ram(ram);
                self.push_stack();
                self.tick_clock();
                self.data_bus = (val & 0x00FF) as u8;
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::POP_rr) => {
                let reg = (ir & 0x30) >> 4;
                self.address_bus = self.register_file.get_sp();
                self.read_ram(ram);
                let val = self.data_bus as u16;
                self.pop_stack();
                self.tick_clock();
                self.read_ram(ram);
                let val = val | ((self.data_bus as u16) << 8);
                self.pop_stack();
                self.tick_clock();
                self.register_file.set16_qq(reg, val).unwrap();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_HL_SPe) => {
                self.read_ram(ram);
                let e = self.data_bus;
                self.increase_pc();
                self.tick_clock();
                let (sum, flags) = ALU::add(self.register_file.get_p(), e);
                self.register_file.set_f(flags & 0x30);
                self.register_file.set_l(sum);
                self.tick_clock();
                let sign = e & 0x80;
                let adj = if sign > 0 { 0xFF } else { 0x00 };
                let (sum, _) = ALU::add3(
                    self.register_file.get_s(),
                    adj,
                    self.register_file.get_carry_flag(),
                );
                self.register_file.set_h(sum);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_r_r) => {
                let target_reg = (ir >> 3) & 0x07;
                let source_reg = ir & 0x07;
                // load content of source to target
                self.register_file
                    .set(target_reg, self.register_file.get(source_reg).unwrap())
                    .unwrap();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_r_HL) => {
                let target_reg = (ir >> 3) & 0x07;
                // load cycle
                let addr = self.register_file.get_hl();
                self.address_bus = addr;
                self.read_ram(ram);
                self.tick_clock();
                // fetch cycle
                self.register_file.set(target_reg, self.data_bus).unwrap();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_HL_r) => {
                let source_reg = ir & 0x07;
                // cycle 1 load address and register content
                let addr = self.register_file.get_hl();
                self.address_bus = addr;
                self.data_bus = self.register_file.get(source_reg).unwrap();
                self.write_ram(ram);
                self.tick_clock();
                // fetch cycle
                self.fetch_cycle(ram);
            }
            Some(OpCode::ADD_r) | Some(OpCode::ADC_r) | Some(OpCode::SUB_r)
            | Some(OpCode::SBC_r) | Some(OpCode::CP_r) | Some(OpCode::AND_r)
            | Some(OpCode::OR_r) | Some(OpCode::XOR_r) => {
                let reg = ir & 0x07;
                let val = self.register_file.get(reg).unwrap();
                match op_code.unwrap() {
                    OpCode::ADD_r => self.add(val, false),
                    OpCode::ADC_r => self.add(val, true),
                    OpCode::SUB_r => self.sub(val, false),
                    OpCode::SBC_r => self.sub(val, true),
                    OpCode::CP_r => {
                        self.compare(val, false);
                    }
                    OpCode::AND_r => self.and(val),
                    OpCode::OR_r => self.or(val),
                    OpCode::XOR_r => self.xor(val),
                    _ => {}
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::ADD_HL) | Some(OpCode::ADC_HL) | Some(OpCode::SUB_HL)
            | Some(OpCode::SBC_HL) | Some(OpCode::CP_HL) | Some(OpCode::AND_HL)
            | Some(OpCode::OR_HL) | Some(OpCode::XOR_HL) => {
                self.address_bus = self.register_file.get_hl();
                self.read_ram(ram);
                self.tick_clock();

                match op_code.unwrap() {
                    OpCode::ADD_HL => self.add(self.data_bus, false),
                    OpCode::ADC_HL => self.add(self.data_bus, true),
                    OpCode::SUB_HL => self.sub(self.data_bus, false),
                    OpCode::SBC_HL => self.sub(self.data_bus, true),
                    OpCode::CP_HL => {
                        self.compare(self.data_bus, false);
                    }
                    OpCode::AND_HL => self.and(self.data_bus),
                    OpCode::OR_HL => self.or(self.data_bus),
                    OpCode::XOR_HL => self.xor(self.data_bus),
                    _ => {}
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::ADD_n) | Some(OpCode::ADC_n) | Some(OpCode::SUB_n)
            | Some(OpCode::SBC_n) | Some(OpCode::CP_n) | Some(OpCode::AND_n)
            | Some(OpCode::OR_n) | Some(OpCode::XOR_n) => {
                self.read_ram(ram);
                self.increase_pc();
                self.tick_clock();
                match op_code.unwrap() {
                    OpCode::ADD_n => self.add(self.data_bus, false),
                    OpCode::ADC_n => self.add(self.data_bus, true),
                    OpCode::SUB_n => self.sub(self.data_bus, false),
                    OpCode::SBC_n => self.sub(self.data_bus, true),
                    OpCode::CP_n => {
                        self.compare(self.data_bus, false);
                    }
                    OpCode::AND_n => self.and(self.data_bus),
                    OpCode::OR_n => self.or(self.data_bus),
                    OpCode::XOR_n => self.xor(self.data_bus),
                    _ => {}
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::INC_r) | Some(OpCode::DEC_r) => {
                let reg = (ir & !opcodes::INC_R_MASK) >> 3;
                let (res, flags) = if op_code.unwrap() == OpCode::INC_r {
                    ALU::increment(self.register_file.get(reg).unwrap())
                } else {
                    ALU::decrement(self.register_file.get(reg).unwrap())
                };
                self.register_file.set(reg, res).unwrap();
                self.register_file.or_flags(flags);
                self.fetch_cycle(ram);
            }
            Some(OpCode::INC_HL) | Some(OpCode::DEC_HL) => {
                self.address_bus = self.register_file.get_hl();
                self.read_ram(ram);
                self.tick_clock();
                let (res, flags) = if op_code.unwrap() == OpCode::INC_HL {
                    ALU::increment(self.data_bus)
                } else {
                    ALU::decrement(self.data_bus)
                };
                self.data_bus = res;
                self.register_file.or_flags(flags & 0xE0);
                self.write_ram(ram);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::CCF) => {
                self.register_file.unset_half_carry_flag();
                self.register_file.unset_negative_flag();
                self.register_file.flip_carry_flag();
                self.fetch_cycle(ram);
            }
            Some(OpCode::SCF) => {
                self.register_file.unset_half_carry_flag();
                self.register_file.unset_negative_flag();
                self.register_file.set_carry_flag();
                self.fetch_cycle(ram);
            }
            Some(OpCode::DAA) => {
                let (res, flags) =
                    ALU::decimal_adjust(self.register_file.get_a(), self.register_file.get_f());
                self.register_file.set_a(res);
                self.register_file.set_f(flags);
                self.fetch_cycle(ram);
            }
            Some(OpCode::CPL) => {
                self.register_file.set_a(!self.register_file.get_a());
                self.register_file.set_negative_flag();
                self.register_file.set_half_carry_flag();
                self.fetch_cycle(ram);
            }
            Some(OpCode::INC_rr) | Some(OpCode::DEC_rr) => {
                let reg = (ir & 0b0011_0000) >> 4;
                self.address_bus = self.register_file.get16_dd(reg).unwrap();
                if op_code.unwrap() == OpCode::INC_rr {
                    self.idu_increment();
                } else {
                    self.idu_decrement();
                }
                self.register_file.set16_dd(reg, self.address_bus).unwrap();
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::ADD_HL_rr) => {
                let reg = (ir & 0b0011_0000) >> 4;
                let v1 = self.register_file.get16_dd(reg).unwrap();

                let lsb_v1 = (v1 & 0x00FF) as u8;
                let msb_v1 = ((v1 & 0xFF00) >> 8) as u8;
                let (res_lsb, flags) = ALU::add(lsb_v1, self.register_file.get_l());
                self.register_file.set_l(res_lsb);
                self.tick_clock();
                let (res_msb, flags) =
                    ALU::add3(msb_v1, self.register_file.get_h(), (flags & 0x10) >> 4);
                self.register_file.set_h(res_msb);
                let previous_zero_flag = self.register_file.get_f() & 0x80;
                self.register_file
                    .set_f(previous_zero_flag | (flags & 0x7F));
                self.fetch_cycle(ram);
            }
            Some(OpCode::ADD_SP_e) => {
                self.read_ram(ram);
                let e = self.data_bus;
                self.increase_pc();
                self.tick_clock();
                let (sum, flags) = ALU::add(self.register_file.get_p(), e);
                let flags = if sum == 0 { flags | 0x80 } else { flags };
                self.register_file.set_f(flags & 0x30);
                self.register_file.set_p(sum);
                self.tick_clock();
                let sign = e & 0x80;
                let adj = if sign > 0 { 0xFF } else { 0x00 };
                let (sum, _) = ALU::add3(
                    self.register_file.get_s(),
                    adj,
                    self.register_file.get_carry_flag(),
                );
                self.register_file.set_s(sum);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::NOP) => {
                self.fetch_cycle(ram);
            }
            Some(OpCode::RLCA) | Some(OpCode::RLA) | Some(OpCode::RRA) | Some(OpCode::RRCA) => {
                let (res, flags) = match op_code.unwrap() {
                    OpCode::RLA => ALU::rotate_left(
                        self.register_file.get_a(),
                        self.register_file.get_carry_flag(),
                    ),
                    OpCode::RLCA => ALU::rotate_left_circular(self.register_file.get_a()),
                    OpCode::RRA => ALU::rotate_right(
                        self.register_file.get_a(),
                        self.register_file.get_carry_flag(),
                    ),
                    OpCode::RRCA => ALU::rotate_right_circular(self.register_file.get_a()),
                    _ => (0, 0),
                };
                self.register_file.set_a(res);
                self.register_file.set_f(flags & 0x10);
                self.fetch_cycle(ram);
            }
            Some(OpCode::CB_PREFIX) => {
                self.fetch_cycle(ram);
                self.tick_clock();
                let cb_ir = self.register_file.get_ir();
                let cb_opcode = CBPrefixOpCode::from_ir(cb_ir);
                //println!("CB opcode: {:?}", cb_opcode);
                match cb_opcode {
                    Some(CBPrefixOpCode::RLC_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) =
                            ALU::rotate_left_circular(self.register_file.get(reg).unwrap());
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::RLC_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) = ALU::rotate_left_circular(self.data_bus);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::RRC_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) =
                            ALU::rotate_right_circular(self.register_file.get(reg).unwrap());
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::RRC_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) = ALU::rotate_right_circular(self.data_bus);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::RL_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) = ALU::rotate_left(
                            self.register_file.get(reg).unwrap(),
                            self.register_file.get_carry_flag(),
                        );
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::RL_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) =
                            ALU::rotate_left(self.data_bus, self.register_file.get_carry_flag());
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::RR_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) = ALU::rotate_right(
                            self.register_file.get(reg).unwrap(),
                            self.register_file.get_carry_flag(),
                        );
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::RR_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) =
                            ALU::rotate_right(self.data_bus, self.register_file.get_carry_flag());
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::SLA_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) =
                            ALU::shift_left_arithmetic(self.register_file.get(reg).unwrap());
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::SLA_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) = ALU::shift_left_arithmetic(self.data_bus);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::SRA_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) =
                            ALU::shift_right_arithmetic(self.register_file.get(reg).unwrap());
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::SRA_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) = ALU::shift_right_arithmetic(self.data_bus);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::SWAP_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) = ALU::swap_nibbles(self.register_file.get(reg).unwrap());
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::SWAP_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) = ALU::swap_nibbles(self.data_bus);
                        self.register_file.set_f(flags);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::SRL_r) => {
                        let reg = cb_ir & 0x07;
                        let (res, flags) =
                            ALU::shift_right_logical(self.register_file.get(reg).unwrap());
                        self.register_file.set(reg, res).unwrap();
                        self.register_file.set_f(flags);
                    }
                    Some(CBPrefixOpCode::SRL_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let (res, flags) = ALU::shift_right_logical(self.data_bus);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.register_file.set_f(flags);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::BIT_b_r) => {
                        let reg = cb_ir & 0x07;
                        let bit = (cb_ir & 0x38) >> 3;
                        let flags = ALU::test_bit(self.register_file.get(reg).unwrap(), bit);
                        //println!("testing bit {} of register {}: {:X}", bit, reg, flags);
                        self.register_file.or_flags(flags & 0xE0);
                    }
                    Some(CBPrefixOpCode::BIT_b_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let bit = (cb_ir & 0x38) >> 3;
                        let flags = ALU::test_bit(self.data_bus, bit);
                        self.register_file.or_flags(flags & 0xE0);
                    }
                    Some(CBPrefixOpCode::SET_b_r) => {
                        let reg = cb_ir & 0x07;
                        let bit = (cb_ir & 0x38) >> 3;
                        let res = ALU::set_bit(self.register_file.get(reg).unwrap(), bit);
                        self.register_file.set(reg, res).unwrap();
                    }
                    Some(CBPrefixOpCode::SET_b_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let bit = (cb_ir & 0x38) >> 3;
                        let res = ALU::set_bit(self.data_bus, bit);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.tick_clock();
                    }
                    Some(CBPrefixOpCode::RES_b_r) => {
                        let reg = cb_ir & 0x07;
                        let bit = (cb_ir & 0x38) >> 3;
                        let res = ALU::reset_bit(self.register_file.get(reg).unwrap(), bit);
                        self.register_file.set(reg, res).unwrap();
                    }
                    Some(CBPrefixOpCode::RES_b_HL) => {
                        self.address_bus = self.register_file.get_hl();
                        self.read_ram(ram);
                        self.tick_clock();
                        let bit = (cb_ir & 0x38) >> 3;
                        let res = ALU::reset_bit(self.data_bus, bit);
                        self.data_bus = res;
                        self.write_ram(ram);
                        self.tick_clock();
                    }
                    None => panic!("Unrecognized CB prefix  op code {:x}", cb_ir),
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::JP_NN) => {
                let val = self.read_16b_ram(ram);
                self.register_file.set_pc(val);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::JP_HL) => {
                self.register_file.set_pc(self.register_file.get_hl());
                self.fetch_cycle(ram);
            }
            Some(OpCode::JP_CC_NN) => {
                let condition = self.code_to_condition((ir >> 3) & 0x03);
                let val = self.read_16b_ram(ram);
                if condition {
                    self.register_file.set_pc(val);
                    self.tick_clock();
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::JR_E) => {
                self.read_ram(ram);
                let val = self.data_bus;
                self.increase_pc();
                self.tick_clock();
                let new_pc = ALU::add_16_signed(self.register_file.get_pc(), val);
                self.tick_clock();
                self.register_file.set_pc(new_pc);
                self.fetch_cycle(ram);
            }
            Some(OpCode::JR_CC_E) => {
                self.read_ram(ram);
                let val = self.data_bus;
                let condition = self.code_to_condition((ir >> 3) & 0x03);
                self.increase_pc();
                self.tick_clock();
                if condition {
                    let new_pc = ALU::add_16_signed(self.register_file.get_pc(), val);
                    self.tick_clock();
                    self.register_file.set_pc(new_pc);
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::CALL_NN) | Some(OpCode::CALL_CC_NN) => {
                let val = self.read_16b_ram(ram);
                let condition = if op_code.unwrap() == OpCode::CALL_CC_NN {
                    self.code_to_condition((ir >> 3) & 0x03)
                } else {
                    true
                };
                if condition {
                    self.push_stack();
                    self.tick_clock();
                    self.data_bus = ((self.register_file.get_pc() & 0xFF00) >> 8) as u8;
                    self.write_ram(ram);
                    self.push_stack();
                    self.tick_clock();
                    self.data_bus = (self.register_file.get_pc() & 0x00FF) as u8;
                    self.write_ram(ram);
                    self.register_file.set_pc(val);
                    self.tick_clock();
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::RET) | Some(OpCode::RET_CC) | Some(OpCode::RETI) => {
                let condition = if op_code == Some(OpCode::RET_CC) {
                    self.tick_clock();
                    self.code_to_condition((ir >> 3) & 0x03)
                } else {
                    true
                };
                if condition {
                    self.address_bus = self.register_file.get_sp();
                    self.read_ram(ram);
                    let lsb = self.data_bus;
                    self.pop_stack();
                    self.tick_clock();
                    self.read_ram(ram);
                    let msb = self.data_bus;
                    self.pop_stack();
                    self.tick_clock();
                    self.register_file
                        .set_pc(((msb as u16) << 8) | (lsb as u16));
                    self.tick_clock();
                }
                if op_code == Some(OpCode::RETI) {
                    self.ime = true;
                }
                self.fetch_cycle(ram);
            }
            Some(OpCode::RST_N) => {
                let code = (ir >> 3) & 0x07;
                let page = self.code_to_page_memory(code);
                self.push_stack();
                self.tick_clock();
                self.data_bus = ((self.register_file.get_pc() & 0xFF00) >> 8) as u8;
                self.write_ram(ram);
                self.push_stack();
                self.tick_clock();
                self.data_bus = (self.register_file.get_pc() & 0x00FF) as u8;
                self.write_ram(ram);
                self.register_file.set_pc(page as u16);
                self.tick_clock();
                self.fetch_cycle(ram);
            }
            Some(OpCode::DI) => {
                self.ime = false;
                self.fetch_cycle(ram);
            }
            Some(OpCode::EI) => {
                self.fetch_cycle(ram);
                self.ime = true;
            }
            Some(OpCode::HALT) => {
                self.fetch_cycle(ram);
                self.address_bus = self.register_file.get_pc();
                self.idu_decrement();
                self.register_file.set_pc(self.address_bus);
            }
            Some(OpCode::STOP) => {
                // TODO implement actual stop function
                self.fetch_cycle(ram);
            }
        }
        self.tick_clock();
    }

    pub fn fps(&self) -> f32 {
        1. / (self.iteration_time as f32 * 1e-9)
    }

    pub fn avg_delay(&self) -> i128 {
        return self.internal_clock.avg_delay();
    }

    pub fn sleep_count(&self) -> u128 {
        return self.internal_clock.sleep_count();
    }

    #[allow(dead_code)]
    pub fn get_register(&self, register: RegisterName) -> u16 {
        match register {
            RegisterName::IR => self.register_file.get_ir() as u16,
            RegisterName::IE => self.register_file.get_ie() as u16,
            RegisterName::AF => self.register_file.get_af(),
            RegisterName::A => self.register_file.get_a() as u16,
            RegisterName::F => self.register_file.get_f() as u16,
            RegisterName::BC => self.register_file.get_bc(),
            RegisterName::B => self.register_file.get_b() as u16,
            RegisterName::C => self.register_file.get_c() as u16,
            RegisterName::DE => self.register_file.get_de(),
            RegisterName::D => self.register_file.get_d() as u16,
            RegisterName::E => self.register_file.get_e() as u16,
            RegisterName::HL => self.register_file.get_hl(),
            RegisterName::H => self.register_file.get_h() as u16,
            RegisterName::L => self.register_file.get_l() as u16,
            RegisterName::PC => self.register_file.get_pc(),
            RegisterName::SP => self.register_file.get_sp(),
        }
    }

    #[allow(dead_code)]
    pub fn get_data_bus(&self) -> u8 {
        self.data_bus
    }

    #[allow(dead_code)]
    pub fn get_address_bus(&self) -> u16 {
        self.address_bus
    }
}
