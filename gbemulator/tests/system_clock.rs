use gbemulator::system::clock::SystemClock;

#[test]
fn test_constructor() {
    let frequency = 4.194304 * 1e6;
    let clock = SystemClock::from_frequency(frequency);
    assert!(clock.period.as_nanos() == 239);
}

#[test]
fn test_next() {
    let frequency = 4.194304;
    let mut clock = SystemClock::from_frequency(frequency);
    clock.next();
    assert!(clock.cycle_count == 1);
    clock.next();
    assert!(clock.cycle_count == 2);
}

#[test]
fn test_delay() {
    let frequency = 4.194304 * 1e6;
    let mut clock = SystemClock::from_frequency(frequency);
    clock.next();
    std::thread::sleep(std::time::Duration::from_nanos(500));
    clock.next();
    assert!(clock.avg_delay() > 0);
}
