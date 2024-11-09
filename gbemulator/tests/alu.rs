use gbemulator::system::sm83::ALU;

#[test]
fn test_add() {
    let v1: u8 = 0b00000000;
    let v2: u8 = 0b00000000;
    let (res, flags) = ALU::add(v1, v2);
    assert_eq!(res, 0);
    assert_eq!(flags, 0);

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
