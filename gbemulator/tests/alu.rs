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

#[test]
fn test_decimal_adjust() {
    let v1 = 0x38;
    let v2 = 0x45;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0x7D);
    assert_eq!(flags, 0);
    let (res, flags) = ALU::decimal_adjust(res, flags & 0x10 > 0, flags & 0x20 > 0);
    assert_eq!(res, 0x83);
    assert_eq!(flags, 0);

    let v1 = 0x11;
    let v2 = 0x22;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0x33);
    assert_eq!(flags, 0);
    let (res, flags) = ALU::decimal_adjust(res, flags & 0x10 > 0, flags & 0x20 > 0);
    assert_eq!(res, 0x33);
    assert_eq!(flags, 0);

    let v1 = 0x18;
    let v2 = 0x08;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0x20);
    assert_eq!(flags, 0x20);
    let (res, flags) = ALU::decimal_adjust(res, flags & 0x10 > 0, flags & 0x20 > 0);
    assert_eq!(res, 0x26);
    assert_eq!(flags, 0);
}

#[test]
fn test_rotate() {
    let v = 0b1000_0001;
    let (res, flags) = ALU::rotate_left(v, 0);
    assert_eq!(res, 0b0000_0010);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::rotate_left(res, flags >> 4);
    assert_eq!(res, 0b0000_0101);
    assert_eq!(flags, 0x00);

    let (res, flags) = ALU::rotate_right(v, 0);
    assert_eq!(res, 0b0100_0000);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::rotate_right(res, flags >> 4);
    assert_eq!(res, 0b1010_0000);
    assert_eq!(flags, 0x00);
}

#[test]
fn test_rotate_circular() {
    let v = 0b1000_0001;
    let (res, flags) = ALU::rotate_left_circular(v);
    assert_eq!(res, 0b0000_0011);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::rotate_left_circular(res);
    assert_eq!(res, 0b0000_0110);
    assert_eq!(flags, 0x00);

    let (res, flags) = ALU::rotate_right_circular(v);
    assert_eq!(res, 0b1100_0000);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::rotate_right_circular(res);
    assert_eq!(res, 0b0110_0000);
    assert_eq!(flags, 0x00);
}
