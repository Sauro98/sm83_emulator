use gbemulator::system::ram::{self};
use gbemulator::system::sm83::opcodes::*;
use gbemulator::system::sm83::registers::RegisterName;
use gbemulator::system::sm83::snapshot::SM83Snapshot;
use gbemulator::system::sm83::{self};

#[tokio::test]
async fn test_ldrn() {
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
async fn test_ldr_r() {
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
async fn test_ldr_hl() {
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
async fn test_ldhlr() {
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
async fn test_ldhln() {
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
async fn test_ldabc() {
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
async fn test_ldbca() {
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
async fn test_ldade() {
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
async fn test_lddea() {
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
async fn test_ldann() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_A_nn).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // n = 0xAA  // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // n = 0x00  // PC = 2 value msb
    ram.set_at(0x0003, 0xCD).unwrap(); //           // PC = 3 dummy value
    ram.set_at(0x00AA, 0xEE).unwrap(); //           // target value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_ldnn_a() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_A_n); // PC = 0 opcode
    ram.set_at(0x0001, 0xEE); // n = 0xEE
    ram.set_at(0x0002, LD_nn_A).unwrap(); // PC = 2 opcode
    ram.set_at(0x0003, 0xAA).unwrap(); // n = 0xAA  // PC = 3 value lsb
    ram.set_at(0x0004, 0x00).unwrap(); // n = 0x00  // PC = 4 value msb
    ram.set_at(0x0005, 0xCD).unwrap(); //           // PC = 5 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.cycle_count, 2);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 6);
}

#[tokio::test]
async fn test_ldhac() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_C_n); // PC = 0 opcode
    ram.set_at(0x0001, 0xEE); // n = 0xEE
    ram.set_at(0x0002, LDH_A_C).unwrap(); // PC = 2 opcode
    ram.set_at(0x0003, 0xCD).unwrap(); // PC = 3 dummy value
    ram.set_at(0xFFEE, 0xBB).unwrap(); // target value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.cycle_count, 2);
    assert_eq!(cpu.get_register(RegisterName::C), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x4);
    assert_eq!(cpu.get_register(RegisterName::A), 0xBB);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_ldhca() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_C_n); // PC = 0 opcode
    ram.set_at(0x0001, 0xEE); // n = 0xEE
    ram.set_at(0x0002, LD_A_n); // PC = 2 opcode
    ram.set_at(0x0003, 0xBB); // n = 0xEE
    ram.set_at(0x0004, LDH_C_A).unwrap(); // PC = 4 opcode
    ram.set_at(0x0005, 0xCD).unwrap(); // PC = 5 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.cycle_count, 2);
    assert_eq!(cpu.get_register(RegisterName::C), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.cycle_count, 4);
    assert_eq!(cpu.get_register(RegisterName::C), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::A), 0xBB);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x6);
    assert_eq!(*ram.get_at(0x0FFEE).unwrap(), 0xBB);
    assert_eq!(cpu.cycle_count, 6);
}

#[tokio::test]
async fn test_ldhan() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LDH_A_n).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xEE).unwrap(); // value
    ram.set_at(0x0002, 0xCD).unwrap(); // PC = 2 dummy value
    ram.set_at(0xFFEE, 0xBB).unwrap(); // target value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 0xBB);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_ldhn_a() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_A_n).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xBB).unwrap(); // value
    ram.set_at(0x0002, LDH_n_A).unwrap(); // PC = 2 opcode
    ram.set_at(0x0003, 0xEE).unwrap(); // value
    ram.set_at(0x0004, 0xCD).unwrap(); // PC = 2 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram);
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0xBB);
    assert_eq!(*ram.get_at(0x0FFEE).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 2);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0xBB);
    assert_eq!(*ram.get_at(0x0FFEE).unwrap(), 0xBB);
    assert_eq!(cpu.cycle_count, 5);
}

#[tokio::test]
async fn test_ldahlm() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // value msb
    ram.set_at(0x0003, LD_A_HLm).unwrap(); // opcode
    ram.set_at(0x0004, 0xCD).unwrap(); // dummy value
    ram.set_at(0x00AA, 0xEE).unwrap(); // value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00A9);
    assert_eq!(cpu.cycle_count, 5);
}

#[tokio::test]
async fn test_ldhlm_a() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // value msb
    ram.set_at(0x0003, LD_A_n).unwrap(); // opcode
    ram.set_at(0x0004, 0xEE).unwrap(); // value
    ram.set_at(0x0005, LD_HLm_A).unwrap(); // opcode
    ram.set_at(0x0006, 0xCD).unwrap(); // dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x07);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00A9);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 7);
}

#[tokio::test]
async fn test_ldahlp() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // value msb
    ram.set_at(0x0003, LD_A_HLp).unwrap(); // opcode
    ram.set_at(0x0004, 0xCD).unwrap(); // dummy value
    ram.set_at(0x00AA, 0xEE).unwrap(); // value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AB);
    assert_eq!(cpu.cycle_count, 5);
}

#[tokio::test]
async fn test_ldhlp_a() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // value msb
    ram.set_at(0x0003, LD_A_n).unwrap(); // opcode
    ram.set_at(0x0004, 0xEE).unwrap(); // value
    ram.set_at(0x0005, LD_HLp_A).unwrap(); // opcode
    ram.set_at(0x0006, 0xCD).unwrap(); // dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0x00);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_data_bus(), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x07);
    assert_eq!(cpu.get_register(RegisterName::A), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AB);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0xEE);
    assert_eq!(cpu.cycle_count, 7);
}

#[tokio::test]
async fn test_ldrrnn() {
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

#[tokio::test]
async fn test_ldsphl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // PC = 2 value msb
    ram.set_at(0x0003, LD_SP_HL).unwrap(); // PC = 3 opcode
    ram.set_at(0x0004, 0xCD).unwrap(); // PC = 4 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_SP_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x0000);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00AA);
    assert_eq!(cpu.cycle_count, 5);
}

#[tokio::test]
async fn test_ldnn_sp() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // PC = 1 value lsb
    ram.set_at(0x0002, 0x02).unwrap(); // PC = 2 value msb
    ram.set_at(0x0003, LD_SP_HL).unwrap(); // PC = 3 opcode
    ram.set_at(0x0004, LD_nn_SP).unwrap(); // PC = 4 opcode
    ram.set_at(0x0005, 0xAB).unwrap(); // PC = 5 value
    ram.set_at(0x0006, 0x01).unwrap(); // PC = 6 value
    ram.set_at(0x0007, 0xCD).unwrap(); // PC = 7 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_SP_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x02AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x0000);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_nn_SP as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x02AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x02AA);
    assert_eq!(*ram.get_at(0x01AB).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x01AC).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x08);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x02AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x02AA);
    assert_eq!(*ram.get_at(0x01AB).unwrap(), 0xAA);
    assert_eq!(*ram.get_at(0x01AC).unwrap(), 0x02);
    assert_eq!(cpu.cycle_count, 10);
}

#[tokio::test]
async fn test_push_rr() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // PC = 2 value msb
    ram.set_at(0x0003, LD_SP_HL).unwrap(); // PC = 3 opcode
    ram.set_at(0x0004, PUSH_HL).unwrap(); // PC = 4 opcode
    ram.set_at(0x0005, 0xCD).unwrap(); // PC = 4 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_SP_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x0000);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), PUSH_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00AA);
    assert_eq!(*ram.get_at(0x00AA).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x00A9).unwrap(), 0x00);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00A8);
    assert_eq!(*ram.get_at(0x00A9).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x00A8).unwrap(), 0xAA);
    assert_eq!(cpu.cycle_count, 9);
}

#[tokio::test]
async fn test_pop_rr() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // PC = 2 value msb
    ram.set_at(0x0003, LD_SP_HL).unwrap(); // PC = 3 opcode
    ram.set_at(0x0004, POP_HL).unwrap(); // PC = 4 opcode
    ram.set_at(0x0005, 0xCD).unwrap(); // PC = 4 dummy value
    ram.set_at(0x00AA, 0xBB).unwrap();
    ram.set_at(0x00AB, 0xCC).unwrap();
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_SP_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x0000);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), POP_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00AA);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x06);
    assert_eq!(cpu.get_register(RegisterName::HL), 0xCCBB);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00AC);
    assert_eq!(cpu.cycle_count, 8);
}

#[tokio::test]
async fn test_ldhl_spe() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, LD_HL_nn).unwrap(); // PC = 0 opcode
    ram.set_at(0x0001, 0xAA).unwrap(); // PC = 1 value lsb
    ram.set_at(0x0002, 0x00).unwrap(); // PC = 2 value msb
    ram.set_at(0x0003, LD_SP_HL).unwrap(); // PC = 3 opcode
    ram.set_at(0x0004, LD_HL_SPe).unwrap(); // PC = 4 opcode
    ram.set_at(0x0005, 0x85).unwrap(); // value
    ram.set_at(0x0006, 0xCD).unwrap(); // PC = 6 dummy value
    let mut cpu = sm83::SM83::new(frequency);
    cpu.fetch_cycle(&mut ram); // fetch, load opcode and advance PC to 1
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_SP_HL as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x04);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x0000);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), LD_HL_SPe as u16);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x05);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x00AA);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00AA);
    assert_eq!(cpu.cycle_count, 5);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x07);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x002F);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x00AA);
    assert_eq!(cpu.cycle_count, 8);
}

#[test]
fn test_load_from_snapshot() {
    // 8 bit
    let snapshot = SM83Snapshot::new()
        .with_address_bus(1)
        .with_data_bus(2)
        .with_a(3)
        .with_b(4)
        .with_c(5)
        .with_d(6)
        .with_e(7)
        .with_f(8)
        .with_h(9)
        .with_l(10)
        .with_sp(11)
        .with_pc(12);
    let frequency = 1. * 1e6;
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    assert_eq!(cpu.get_address_bus(), 1);
    assert_eq!(cpu.get_data_bus(), 2);
    assert_eq!(cpu.get_register(RegisterName::A), 3);
    assert_eq!(cpu.get_register(RegisterName::B), 4);
    assert_eq!(cpu.get_register(RegisterName::C), 5);
    assert_eq!(cpu.get_register(RegisterName::D), 6);
    assert_eq!(cpu.get_register(RegisterName::E), 7);
    assert_eq!(cpu.get_register(RegisterName::F), 8);
    assert_eq!(cpu.get_register(RegisterName::H), 9);
    assert_eq!(cpu.get_register(RegisterName::L), 10);
    assert_eq!(cpu.get_register(RegisterName::SP), 11);
    assert_eq!(cpu.get_register(RegisterName::PC), 12);

    //16 bit
    let snapshot = SM83Snapshot::new()
        .with_af(0x1234)
        .with_bc(0x5678)
        .with_de(0x9ABC)
        .with_hl(0xDEF0);
    let frequency = 1. * 1e6;
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    assert_eq!(cpu.get_register(RegisterName::AF), 0x1234);
    assert_eq!(cpu.get_register(RegisterName::BC), 0x5678);
    assert_eq!(cpu.get_register(RegisterName::DE), 0x9ABC);
    assert_eq!(cpu.get_register(RegisterName::HL), 0xDEF0);
}

#[tokio::test]
async fn test_add_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADD_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(3).with_b(4);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 7);
    assert_eq!(cpu.get_register(RegisterName::F), 0);
    assert_eq!(cpu.cycle_count, 1);

    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADD_C).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(15).with_c(1);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0x10);
    assert_eq!(cpu.get_register(RegisterName::F), 0x20);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_add_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADD_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(3).with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 8);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_add_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADD_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(5);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 0x0A);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_adc_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADC_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(3).with_b(4).with_f(0x10);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 8);
    assert_eq!(cpu.get_register(RegisterName::F), 0);
    assert_eq!(cpu.cycle_count, 1);

    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADC_C).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(15).with_c(1).with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0x10);
    assert_eq!(cpu.get_register(RegisterName::F), 0x20);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_adc_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADC_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(3).with_hl(0xABCD).with_f(0x10);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 9);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_adc_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADC_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(5).with_f(0x10);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 0x0B);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_sub_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SUB_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(4).with_b(3);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 1);
    assert_eq!(cpu.get_register(RegisterName::F), 0x70);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_sub_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SUB_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(7).with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 2);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_sub_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SUB_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(255);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 250);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_sbc_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SBC_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(16).with_b(4).with_f(0x10);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 11);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_sbc_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SBC_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(255).with_hl(0xABCD).with_f(0x10);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 249);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_sbc_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SBC_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(5).with_f(0x10);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 255);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_cp_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CP_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(4).with_b(3);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 4);
    assert_eq!(cpu.get_register(RegisterName::F), 0x70);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_cp_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CP_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(7).with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 7);
    assert_eq!(cpu.get_register(RegisterName::F), 0x70);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_cp_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CP_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(5);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 5);
    assert_eq!(cpu.get_register(RegisterName::F), 0xF0);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_inc_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, INC_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(1);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::B), 2);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 1);

    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, INC_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(15);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::B), 16);
    assert_eq!(cpu.get_register(RegisterName::F), 0x20);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_inc_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, INC_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xEEEE, 11).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xEEEE);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(*ram.get_at(0xEEEE).unwrap(), 12);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_dec_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, DEC_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(1);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::B), 0);
    assert_eq!(cpu.get_register(RegisterName::F), 0xE0);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_dec_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, DEC_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xEEEE, 11).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xEEEE);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x60);
    assert_eq!(*ram.get_at(0xEEEE).unwrap(), 10);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_and_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, AND_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b0010).with_b(0b0110);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0010);
    assert_eq!(cpu.get_register(RegisterName::F), 0x20);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_and_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, AND_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(7).with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 5);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_and_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, AND_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(254);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 4);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_or_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, OR_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b0010).with_b(0b0110);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0110);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_or_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, OR_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(7).with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 7);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_or_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, OR_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(254);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 255);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_xor_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, XOR_B).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b0010).with_b(0b0110);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0100);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_xor_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, XOR_HL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0xABCD, 0x05).unwrap();
    let snapshot = SM83Snapshot::new().with_a(7).with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 2);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_xor_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, XOR_n).unwrap();
    ram.set_at(0x0001, 0x05).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(254);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::A), 251);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_ccf() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CCF).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_f(0xF0);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x80);
    assert_eq!(cpu.cycle_count, 1);

    let snapshot = SM83Snapshot::new().with_f(0xE0);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x90);
    assert_eq!(cpu.cycle_count, 1);

    let snapshot = SM83Snapshot::new().with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x10);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_scf() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, SCF).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_f(0xF0);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x90);
    assert_eq!(cpu.cycle_count, 1);

    let snapshot = SM83Snapshot::new().with_f(0xE0);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x90);
    assert_eq!(cpu.cycle_count, 1);

    let snapshot = SM83Snapshot::new().with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::F), 0x10);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_daa() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, DAA).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0x7D).with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0x83);
    assert_eq!(cpu.get_register(RegisterName::F), 0x00);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_cpl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CPL).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b1010_0101).with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0101_1010);
    assert_eq!(cpu.get_register(RegisterName::F), 0x60);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_inc_rr() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, INC_BC).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_bc(15);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::BC), 16);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_dec_rr() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, DEC_BC).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_bc(15);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::BC), 14);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_add_hl_rr() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADD_HL_BC).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0x00FF).with_bc(0x0101);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::HL), 0x0200);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_add_spe() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, ADD_SP_e).unwrap();
    ram.set_at(0x0001, 0x85).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x00AA);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x002F);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_rlca() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, RLCA).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0000_0001);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_rla() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, RLA).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_rrca() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, RRCA).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b0000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b1000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_rra() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, RRA).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_a(0b0000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x02);
    assert_eq!(cpu.get_register(RegisterName::A), 0b0000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_rlc_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RLC_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0000_0001);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_rlc_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RLC_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0000).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0000_0001);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_rl_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RL_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_rl_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RL_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0000).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_rrc_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RRC_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0000_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_rrc_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RRC_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0000).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0000_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_rr_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RR_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b0000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_rr_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RR_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_sla_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SLA_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0000_0010);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_sla_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SLA_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0000_0010);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_sra_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SRA_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b1100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_sra_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SRA_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b1100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_swap_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SWAP_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0001_1000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_swap_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SWAP_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0001_1000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_srl_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SRL_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b0100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_srl_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SRL_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b0100_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0001_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_bit_b_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, BIT_0_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b1000_0001);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0010_0000);
    assert_eq!(cpu.cycle_count, 2);

    let snapshot = SM83Snapshot::new().with_b(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b1000_0000);
    assert_eq!(cpu.get_register(RegisterName::F), 0b1010_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_bit_b_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, BIT_0_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::F), 0b0010_0000);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_res_b_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RES_0_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0001);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b1000_0000);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_res_b_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, RES_0_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0001).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b1000_0000);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_set_b_r() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SET_0_B).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_b(0b1000_0000);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(cpu.get_register(RegisterName::B), 0b1000_0001);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_set_b_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CB_PREFIX).unwrap();
    ram.set_at(0x0001, SET_0_HL).unwrap();
    ram.set_at(0x0002, 0xCD).unwrap();
    ram.set_at(0xABCD, 0b1000_0000).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x03);
    assert_eq!(*ram.get_at(0xABCD).unwrap(), 0b1000_0001);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_jp_nn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, JP_NN).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0x0002, 0xAB).unwrap();
    ram.set_at(0xABCD, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new();
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0xABCE);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_jp_hl() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, JP_HL).unwrap();
    ram.set_at(0xABCD, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_hl(0xABCD);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0xABCE);
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_jp_cc_nn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, JP_NZ_NN).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    ram.set_at(0x0002, 0xAB).unwrap();
    ram.set_at(0x0003, 0xEE).unwrap();
    ram.set_at(0xABCD, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_f(0x80);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::PC), 0x4);
    assert_eq!(cpu.cycle_count, 3);

    let snapshot = SM83Snapshot::new().with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::PC), 0xABCE);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_jr_e() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0008, JR_E).unwrap();
    ram.set_at(0x0009, 0xFD).unwrap(); // -3 in 2's complement
    ram.set_at(0x0007, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_pc(0x0008);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 8);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_jr_cc_e() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0008, JR_NZ_E).unwrap();
    ram.set_at(0x0009, 0xFD).unwrap(); // -3 in 2's complement
    ram.set_at(0x0007, 0xCD).unwrap();
    ram.set_at(0x000A, 0xBB).unwrap();
    let snapshot = SM83Snapshot::new().with_pc(0x0008).with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 8);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.cycle_count, 3);

    let snapshot = SM83Snapshot::new().with_pc(0x0008).with_f(0x80);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 11);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xBB);
    assert_eq!(cpu.cycle_count, 2);
}

#[tokio::test]
async fn test_call_nn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CALL_NN).unwrap();
    ram.set_at(0x0001, 0xBB).unwrap();
    ram.set_at(0x0002, 0xAA).unwrap();
    ram.set_at(0xAABB, 0xEE).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x1234);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABC);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x03);
    assert_eq!(cpu.cycle_count, 6);
}

#[tokio::test]
async fn test_call_cc_nn() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CALL_NZ_NN).unwrap();
    ram.set_at(0x0001, 0xBB).unwrap();
    ram.set_at(0x0002, 0xAA).unwrap();
    ram.set_at(0x0003, 0x11).unwrap();
    ram.set_at(0xAABB, 0xEE).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x1234).with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABC);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xEE);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x03);
    assert_eq!(cpu.cycle_count, 6);

    let snapshot = SM83Snapshot::new().with_sp(0x1234).with_f(0x80);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0x4);
    assert_eq!(cpu.get_register(RegisterName::IR), 0x11);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1234);
    assert_eq!(cpu.cycle_count, 3);
}

#[tokio::test]
async fn test_ret() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CALL_NN).unwrap();
    ram.set_at(0x0001, 0xBB).unwrap();
    ram.set_at(0x0002, 0xAA).unwrap();
    ram.set_at(0x0003, 0xDD).unwrap();
    ram.set_at(0xAABB, RET).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x1234);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABC);
    assert_eq!(cpu.get_register(RegisterName::IR), RET as u16);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x03);
    assert_eq!(cpu.cycle_count, 6);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0x0004);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xDD);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1234);
    assert_eq!(cpu.cycle_count, 10);
}

#[tokio::test]
async fn test_ret_cc() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CALL_NN).unwrap();
    ram.set_at(0x0001, 0xBB).unwrap();
    ram.set_at(0x0002, 0xAA).unwrap();
    ram.set_at(0x0003, 0xDD).unwrap();
    ram.set_at(0xAABB, RET_NZ).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x1234).with_f(0x00);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABC);
    assert_eq!(cpu.get_register(RegisterName::IR), RET_NZ as u16);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x03);
    assert_eq!(cpu.cycle_count, 6);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0x0004);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xDD);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1234);
    assert_eq!(cpu.cycle_count, 11);

    let snapshot = SM83Snapshot::new().with_sp(0x1234).with_f(0x80);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABC);
    assert_eq!(cpu.get_register(RegisterName::IR), RET_NZ as u16);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x03);
    assert_eq!(cpu.cycle_count, 6);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABD);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(cpu.cycle_count, 8);
}

#[tokio::test]
async fn test_reti() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, CALL_NN).unwrap();
    ram.set_at(0x0001, 0xBB).unwrap();
    ram.set_at(0x0002, 0xAA).unwrap();
    ram.set_at(0x0003, 0xDD).unwrap();
    ram.set_at(0xAABB, RETI).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x1234);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0xAABC);
    assert_eq!(cpu.get_register(RegisterName::IR), RETI as u16);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x03);
    assert_eq!(cpu.cycle_count, 6);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0x0004);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xDD);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1234);
    assert!(cpu.interrupt_enabled());
    assert_eq!(cpu.cycle_count, 10);
}

#[tokio::test]
async fn test_rst_n() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, RST_18).unwrap();
    ram.set_at(0x0001, 0xCC).unwrap();
    ram.set_at(0x0018, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_sp(0x1234);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 0x0019);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert_eq!(cpu.get_register(RegisterName::SP), 0x1232);
    assert_eq!(*ram.get_at(0x1233).unwrap(), 0x00);
    assert_eq!(*ram.get_at(0x1232).unwrap(), 0x01);
    assert_eq!(cpu.cycle_count, 4);
}

#[tokio::test]
async fn test_di() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, DI).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_ime(true);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 2);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert!(!cpu.interrupt_enabled());
    assert_eq!(cpu.cycle_count, 1);
}

#[tokio::test]
async fn test_ei() {
    let frequency = 1. * 1e6;
    let mut ram = ram::RAM::new();
    ram.set_at(0x0000, EI).unwrap();
    ram.set_at(0x0001, 0xCD).unwrap();
    let snapshot = SM83Snapshot::new().with_ime(false);
    let mut cpu = sm83::SM83::new(frequency);
    cpu.load_snapshot(snapshot);
    cpu.fetch_cycle(&mut ram);
    cpu.next(&mut ram).await;
    assert_eq!(cpu.get_register(RegisterName::PC), 2);
    assert_eq!(cpu.get_register(RegisterName::IR), 0xCD);
    assert!(cpu.interrupt_enabled());
    assert_eq!(cpu.cycle_count, 1);
}
