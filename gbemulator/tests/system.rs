use gbemulator::system::System;

fn test_next() {
    let frequency = 4.194304;
    let mut system = System::new(frequency, None);
    system.next();
    system.next();
    assert_eq!(system.cycle_count(), 2);
}
