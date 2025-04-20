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
    assert_eq!(flags, 0b0100_0000);
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
    assert_eq!(f, 0b1100_0000);

    let v = 0b0000_0000;
    let (r, f) = ALU::decrement(v);
    assert_eq!(r, 0b1111_1111);
    assert_eq!(f, 0b0110_0000);

    let v = 0b0000_0100;
    let (r, f) = ALU::decrement(v);
    assert_eq!(r, 0b0000_0011);
    assert_eq!(f, 0b0100_0000);
}

#[test]
fn test_decimal_adjust() {
    let v1 = 0x38;
    let v2 = 0x45;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0x7D);
    assert_eq!(flags, 0);
    let (res, flags) = ALU::decimal_adjust(res, flags);
    assert_eq!(res, 0x83);
    assert_eq!(flags, 0);

    let v1 = 0x11;
    let v2 = 0x22;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0x33);
    assert_eq!(flags, 0);
    let (res, flags) = ALU::decimal_adjust(res, flags);
    assert_eq!(res, 0x33);
    assert_eq!(flags, 0);

    let v1 = 0x18;
    let v2 = 0x08;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0x20);
    assert_eq!(flags, 0x20);
    let (res, flags) = ALU::decimal_adjust(res, flags);
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

#[test]
fn test_shift_arithmetic() {
    let v = 0b1000_0001;
    let (res, flags) = ALU::shift_left_arithmetic(v);
    assert_eq!(res, 0b0000_0010);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::shift_left_arithmetic(res);
    assert_eq!(res, 0b0000_0100);
    assert_eq!(flags, 0x00);

    let (res, flags) = ALU::shift_right_arithmetic(v);
    assert_eq!(res, 0b1100_0000);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::shift_right_arithmetic(res);
    assert_eq!(res, 0b1110_0000);
    assert_eq!(flags, 0x00);
}

#[test]
fn test_shift_logical() {
    let v = 0b1000_0001;
    let (res, flags) = ALU::shift_right_logical(v);
    assert_eq!(res, 0b0100_0000);
    assert_eq!(flags, 0x10);
    let (res, flags) = ALU::shift_right_logical(res);
    assert_eq!(res, 0b0010_0000);
    assert_eq!(flags, 0x00);
}

#[test]
fn test_swap_nibbles() {
    let v = 0b1000_0001;
    let (res, _) = ALU::swap_nibbles(v);
    assert_eq!(res, 0b0001_1000);
    let v = 0xAB;
    let (res, _) = ALU::swap_nibbles(v);
    assert_eq!(res, 0xBA);
}

#[test]
fn test_test_bit() {
    let v = 0b1001_0110;
    assert_eq!(ALU::test_bit(v, 0), 0xA0);
    assert_eq!(ALU::test_bit(v, 1), 0x20);
    assert_eq!(ALU::test_bit(v, 2), 0x20);
    assert_eq!(ALU::test_bit(v, 3), 0xA0);
}

#[test]
fn test_set_bit() {
    let v = 0b0000_0110;
    assert_eq!(ALU::set_bit(v, 0), 0b0000_0111);
    assert_eq!(ALU::set_bit(v, 1), 0b0000_0110);
    assert_eq!(ALU::set_bit(v, 2), 0b0000_0110);
    assert_eq!(ALU::set_bit(v, 3), 0b0000_1110);
    assert_eq!(ALU::set_bit(v, 7), 0b1000_0110);
}

#[test]
fn test_reset_bit() {
    let v = 0b1000_0110;
    assert_eq!(ALU::reset_bit(v, 0), 0b1000_0110);
    assert_eq!(ALU::reset_bit(v, 1), 0b1000_0100);
    assert_eq!(ALU::reset_bit(v, 2), 0b1000_0010);
    assert_eq!(ALU::reset_bit(v, 3), 0b1000_0110);
    assert_eq!(ALU::reset_bit(v, 7), 0b0000_0110);
}
