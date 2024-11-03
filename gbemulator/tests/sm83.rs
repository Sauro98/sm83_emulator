#[macro_use]
use gbemulator::system::opcodes::{*};
use gbemulator::system::ram::{self, RAM};
use gbemulator::system::registers::RegisterName;
use gbemulator::system::sm83::{self, SM83};

#[tokio::test]
async fn test_LDrn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_B_n).unwrap(); // LD B,n    // PC = 0 opcode
    ram.set_at(0x0001, 0xAB).unwrap(); // n = 0xAB  // PC = 1 value
    ram.set_at(0x0002, 0xCD).unwrap(); //           // PC = 2 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    assert_eq!(cpu.get_data_bus(), LD_B_n);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_B_n as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x01);
    assert_eq!(cpu.get_register(RegisterName::B), 0x00);
    assert_eq!(cpu.cycle_count, 0);
    cpu.next(&mut ram).await; // cycle 1: run, read value, increase PC to 2
                              // cycle 2: write value, read PC to IR, increase PC to 3, ready to execute instruction and PC 2
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0xAB);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_LDrR() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_C_n).unwrap(); // LD C,n    // PC = 0 opcode
    ram.set_at(0x0001, 0xAB).unwrap(); // n = 0xAB  // PC = 1 value
    ram.set_at(0x0002, LD_B_C).unwrap(); // LD B,C    // PC = 2 opcode
    ram.set_at(0x0003, 0xCD).unwrap(); //           // PC = 3 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    assert_eq!(cpu.get_data_bus(), LD_C_n);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_C_n as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x01);
    assert_eq!(cpu.get_register(RegisterName::C), 0x00);
    assert_eq!(cpu.cycle_count, 0);
    cpu.next(&mut ram).await; // cycle 1: run, read value, increase PC to 2
                              // cycle 2: write value, read PC to IR, increase PC to 3, ready to execute instruction and PC 2
    assert_eq!(cpu.get_data_bus(), LD_B_C);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_B_C as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::C), 0xAB);
    assert_eq!(cpu.get_register(RegisterName::B), 0x00);
    assert_eq!(cpu.cycle_count, 2);
    cpu.next(&mut ram).await; // cycle 1: run, read value, copy C to B
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::C), 0xAB);
    assert_eq!(cpu.get_register(RegisterName::B), 0xAB);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_LDrHL() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // LD HL,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0x0A).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_B_HL).unwrap(); // LD B,HL   // PC = 3 opcode
    ram.set_at(0x0004, 0xCD).unwrap(); //           // PC = 4 dummy value
    ram.set_at(0x000A, 0xEE).unwrap(); //           // PC = 10 target value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), LD_B_HL);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_B_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0x0A);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x000A);
    assert_eq!(cpu.cycle_count, 3);
    cpu.next(&mut ram).await; // cycle 4: run, read value
                              // cycle 5: copy value to B, read pc to IR, increase PC to 5, ready to execute instruction and PC 4
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0x0A);
    assert_eq!(cpu.get_register(RegisterName::B), 0xEE);
    assert_eq!(cpu.cycle_count, 5);
}

#[tokio::test]
async fn test_LDHLr() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // LD HL,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0x0A).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_B_n).unwrap(); // LD B,n    // PC = 3 opcode
    ram.set_at(0x0004, 0xEE).unwrap(); // n = 0xEE  // PC = 4 value
    ram.set_at(0x0005, LD_HL_B).unwrap(); // LD HL,B   // PC = 5 opcode
    ram.set_at(0x0006, 0xCD).unwrap(); //           // PC = 6 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    cpu.next(&mut ram).await; // cycle 4: run, read value, increase PC to 5
                              // cycle 5: write value, read PC to IR, increase PC to 6, ready to execute instruction and PC 5
    assert_eq!(cpu.get_data_bus(), LD_HL_B);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_HL_B as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0x0A);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x000A);
    assert_eq!(cpu.get_register(RegisterName::B), 0xEE);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await; // cycle 6: run, read value, copy to memory
                              // cycle 7: copy value to HL address, read pc to IR, increase PC to 7, ready to execute instruction and PC 6
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x07);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0x0A);
    assert_eq!(cpu.get_register(RegisterName::B), 0xEE);
    assert_eq!(*ram.get_at(0x000A).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 7);
}

#[tokio::test]
async fn test_LDHLn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // LD HL,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0x0A).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_HL_n).unwrap(); // LD HL,n   // PC = 3 opcode
    ram.set_at(0x0004, 0xEE).unwrap(); // n = 0xEE  // PC = 4 value
    ram.set_at(0x0005, 0xCD).unwrap(); //           // PC = 5 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), LD_HL_n);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_HL_n as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0x0A);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x000A);
    assert_eq!(cpu.cycle_count, 3);
    cpu.next(&mut ram).await; // cycle 4: run, read value from ram
                              // cycle 5: write value to ram
                              // cycle 6: copy value to ram, read pc to IR, increase PC to 6, ready to execute instruction and PC 5
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0x0A);
    assert_eq!(*ram.get_at(0x000A).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 6);
}

#[tokio::test]
async fn test_LDABC() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_BC_nn).unwrap(); // LD BC,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_A_BC).unwrap(); // LD A,BC   // PC = 3 opcode
    ram.set_at(0x0004, 0xCD).unwrap(); //           // PC = 4 dummy value
    ram.set_at(0x00AA, 0xEE).unwrap(); //           // PC = AA target value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), LD_A_BC);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_A_BC as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::B), 0x00);
    assert_eq!(cpu.get_register(RegisterName::C), 0xAA);
    assert_eq!(cpu.get_register(RegisterName::BC), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(cpu.cycle_count, 3);
    cpu.next(&mut ram).await; // cycle 1: run, read value from ram
                              // cycle 2: copy value, read PC to IR, increase PC to 5, ready to execute instruction and PC 4
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
}

#[tokio::test]
async fn test_LDBCA() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_BC_nn).unwrap(); // LD BC,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_A_n).unwrap(); // LD A,n    // PC = 3 opcode
    ram.set_at(0x0004, 0xEE).unwrap(); //           // PC = 4 value
    ram.set_at(0x0005, LD_BC_A).unwrap(); //           // PC = 5 opcode
    ram.set_at(0x0006, 0xCD).unwrap(); //           // PC = 6 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), LD_A_n);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_A_n as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::B), 0x00);
    assert_eq!(cpu.get_register(RegisterName::C), 0xAA);
    assert_eq!(cpu.get_register(RegisterName::BC), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 3);
    cpu.next(&mut ram).await; // cycle 4: run, read value from ram increase pc to 5
                              // cycle 5: copy value, read PC to IR, increase PC to 6, ready to execute instruction and PC 5
    assert_eq!(cpu.get_data_bus(), LD_BC_A);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_BC_A as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await; // cycle 6: run, write value to ram
                              // cycle 7: read PC to IR, increase PC to 7, ready to execute instruction and PC 6
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x07);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 7);
}

#[tokio::test]
async fn test_LDADE() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_DE_nn).unwrap(); // LD DE,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_A_DE).unwrap(); // LD A,DE   // PC = 3 opcode
    ram.set_at(0x0004, 0xCD).unwrap(); //           // PC = 4 dummy value
    ram.set_at(0x00AA, 0xEE).unwrap(); //           // PC = AA target value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), LD_A_DE);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_A_DE as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::D), 0x00);
    assert_eq!(cpu.get_register(RegisterName::E), 0xAA);
    assert_eq!(cpu.get_register(RegisterName::DE), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(cpu.cycle_count, 3);
    cpu.next(&mut ram).await; // cycle 1: run, read value from ram
                              // cycle 2: copy value, read PC to IR, increase PC to 5, ready to execute instruction and PC 4
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
}

#[tokio::test]
async fn test_LDDEA() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_DE_nn).unwrap(); // LD DE,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, LD_A_n).unwrap(); // LD A,n    // PC = 3 opcode
    ram.set_at(0x0004, 0xEE).unwrap(); //           // PC = 4 value
    ram.set_at(0x0005, LD_DE_A).unwrap(); //           // PC = 5 opcode
    ram.set_at(0x0006, 0xCD).unwrap(); //           // PC = 6 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), LD_A_n);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_A_n as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::D), 0x00);
    assert_eq!(cpu.get_register(RegisterName::E), 0xAA);
    assert_eq!(cpu.get_register(RegisterName::DE), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 3);
    cpu.next(&mut ram).await; // cycle 4: run, read value from ram increase pc to 5
                              // cycle 5: copy value, read PC to IR, increase PC to 6, ready to execute instruction and PC 5
    assert_eq!(cpu.get_data_bus(), LD_DE_A);
    assert_eq!(cpu.get_register(RegisterName::IR), LD_DE_A as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await; // cycle 6: run, write value to ram
                              // cycle 7: read PC to IR, increase PC to 7, ready to execute instruction and PC 6
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x07);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 7);
}

#[tokio::test]
async fn test_LDrrnn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // LD HL,nn  // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, 0xCD).unwrap(); //           // PC = 3 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await; // cycle 1: run, read value lsb, increase PC to 2
                              // cycle 2: run, read value msb, increase PC to 3
                              // cycle 3: write value, read PC to IR, increase PC to 4, ready to execute instruction and PC 3
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::H), 0x00);
    assert_eq!(cpu.get_register(RegisterName::L), 0xAA);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.cycle_count, 3);
}
