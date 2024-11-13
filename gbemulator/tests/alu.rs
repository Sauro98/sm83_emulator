use gbemulator::system::sm83::alu::ALU;

#[test]
fn test_add() {
    let v1: u8 = 0b00000000;
    let v2: u8 = 0b00000000;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0);
    assert_eq!(flags, 0x80);

    let v1: u8 = 0b00001000;
    let v2: u8 = 0b00001000;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0b00010000);
    assert_eq!(flags, 0b00100000);

    let v1: u8 = 0b10001000;
    let v2: u8 = 0b10001000;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0b00010000);
    println!("{}", flags);
    assert_eq!(flags, 0b00110000);
}

#[test]
fn test_sub() {
    let v1: u8 = 0b00000000;
    let v2: u8 = 0b00000000;
    let (res, flags) = ALU::sub(v1, v2);
    assert_eq!(res, 0);
    assert_eq!(flags, 0xC0);

    let v1: u8 = 0b00001000;
    let v2: u8 = 0b00000110;
    let (res, flags) = ALU::sub(v1, v2);
    assert_eq!(res, 0b00000010);
    assert_eq!(flags, 0b0111_0000);
}

#[test]
fn test_increment() {
    let v = 0b1111_1111;
    let (r, f) = ALU::increment(v);
    assert_eq!(r, 0b0000_0000);
    assert_eq!(f, 0b1010_0000);

    let v = 0b0000_0100;
    let (r, f) = ALU::increment(v);
    assert_eq!(r, 0b0000_0101);
    assert_eq!(f, 0);

    let v = 0b0000_1111;
    let (r, f) = ALU::increment(v);
    assert_eq!(r, 0b0001_0000);
    assert_eq!(f, 0b0010_0000);
}

#[test]
fn test_decrement() {
    let v = 0b0000_0001;
    let (r, f) = ALU::decrement(v);
    assert_eq!(r, 0b0000_0000);
    assert_eq!(f, 0b1110_0000);

    let v = 0b0000_0000;
    let (r, f) = ALU::decrement(v);
    assert_eq!(r, 0b1111_1111);
    assert_eq!(f, 0b0100_0000);

    let v = 0b0000_0100;
    let (r, f) = ALU::decrement(v);
    assert_eq!(r, 0b0000_0011);
    assert_eq!(f, 0b0110_0000);
}
