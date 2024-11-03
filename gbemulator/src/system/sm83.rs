use crate::system::clock::SystemClock;
use crate::system::opcodes::OpCode;
use crate::system::ram::RAM;
use crate::system::registers::{RegisterFile, RegisterName};

struct ALU {
    output: u8,
}

struct IDU {
    output: u16,
}

impl IDU {
    pub fn increment(&mut self, address: u16) {
        self.output = address + 1;
    }

    pub fn decrement(&mut self, address: u16) {
        self.output = address - 1;
    }
}

pub struct SM83 {
    internal_clock: SystemClock,
    last_execution_time: tokio::time::Instant,
    iteration_time: u128,
    pub cycle_count: u128,
    alu: ALU,
    idu: IDU,
    register_file: RegisterFile,
    address_bus: u16,
    data_bus: u8,
}

impl SM83 {
    pub fn new(clock_frequency: f32) -> SM83 {
        SM83 {
            internal_clock: SystemClock::from_frequency(clock_frequency),
            last_execution_time: tokio::time::Instant::now(),
            iteration_time: 1,
            cycle_count: 0,
            alu: ALU { output: 0 },
            idu: IDU { output: 0 },
            register_file: RegisterFile::new(),
            address_bus: 0,
            data_bus: 0,
        }
    }

    fn increase_PC(&mut self) {
        self.idu.increment(self.register_file.get_pc());
        self.register_file.set_pc(self.idu.output);
        //if self.register_file.get_pc() > 7999 {
        //    self.register_file.set_pc(0);
        //}
        self.address_bus = self.register_file.get_pc();
    }

    pub fn fetch_cycle(&mut self, ram: &RAM) {
        self.address_bus = self.register_file.get_pc();
        self.read_ram(ram);
        self.register_file.set_ir(self.data_bus);
        self.increase_PC();
    }

    fn read_ram(&mut self, ram: &RAM) {
        self.data_bus = *ram.get_at(self.address_bus).unwrap();
    }

    fn write_ram(&self, ram: &mut RAM) {
        ram.set_at(self.address_bus, self.data_bus).unwrap();
    }

    async fn tick_clock(&mut self) {
        self.internal_clock.next().await;
        let duration = tokio::time::Instant::now().duration_since(self.last_execution_time);
        if self.cycle_count == 0 {
            self.iteration_time = duration.as_nanos();
        } else {
            self.iteration_time = (self.iteration_time + duration.as_nanos()) / 2;
        }
        self.cycle_count += 1;
        self.last_execution_time = tokio::time::Instant::now();
    }

    pub async fn next(&mut self, ram: &mut RAM) {
        let ir = self.register_file.get_ir();
        let op_code = OpCode::from_ir(ir);

        match op_code {
            None => {
                println!("Unrecognized OP CODE {}", ir);
            }
            Some(OpCode::LD_HL_n) => {
                // read value from ram
                self.read_ram(ram);
                self.increase_PC();
                self.tick_clock().await;
                //  write value to ram
                self.address_bus = self.register_file.get_hl();
                self.write_ram(ram);
                self.tick_clock().await;
                // fetch cycle
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_A_BC) => {
                // read value from ram
                self.address_bus = self.register_file.get_bc();
                self.read_ram(ram);
                self.tick_clock().await;
                // fetch cycle
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_A_DE) => {
                // read value from ram
                self.address_bus = self.register_file.get_de();
                self.read_ram(ram);
                self.tick_clock().await;
                // fetch cycle
                self.register_file.set_a(self.data_bus);
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_BC_A) => {
                self.address_bus = self.register_file.get_bc();
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock().await;
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_DE_A) => {
                self.address_bus = self.register_file.get_de();
                self.data_bus = self.register_file.get_a();
                self.write_ram(ram);
                self.tick_clock().await;
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_r_n) => {
                // data read cycle
                let reg = (ir >> 3) & 0x07;
                self.read_ram(ram);
                self.increase_PC();
                self.tick_clock().await;
                // fetch cycle
                self.register_file.set(reg, self.data_bus).unwrap();
                self.fetch_cycle(ram);
            }
            Some(OpCode::LD_rr_nn) => {
                let reg = ir >> 4 & 0x03;
                self.read_ram(ram);
                let value = self.data_bus as u16;
                self.increase_PC();
                self.tick_clock().await;
                self.read_ram(ram);
                let value = ((self.data_bus as u16) << 8) | value;
                self.increase_PC();
                self.tick_clock().await;
                self.register_file.set16(reg, value).unwrap();
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
                self.tick_clock().await;
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
                self.tick_clock().await;
                // fetch cycle
                self.fetch_cycle(ram);
            }
            Some(x) => {
                println!("OPCODE not yet implemented {:?}", x);
            }
        }
        self.tick_clock().await;
    }

    pub fn fps(&self) -> f32 {
        1. / (self.iteration_time as f32 * 1e-9)
    }

    pub fn avg_delay(&self) -> i128 {
        return self.internal_clock.avg_delay();
    }

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

    pub fn get_data_bus(&self) -> u8 {
        self.data_bus
    }

    pub fn get_address_bus(&self) -> u16 {
        self.address_bus
    }
}
