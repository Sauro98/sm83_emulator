use gbemulator::system::clock::SystemClock;

#[test]
fn test_constructor() {
    let frequency = 4.194304 * 1e6;
    let clock = SystemClock::from_frequency(frequency);
    assert!(clock.period.as_nanos() == 239);
}

#[tokio::test]
async fn test_next() {
    let frequency = 4.194304;
    let mut clock = SystemClock::from_frequency(frequency);
    clock.next().await;
    assert!(clock.cycle_count == 1);
    clock.next().await;
    assert!(clock.cycle_count == 2);
    assert!(clock.avg_delay() == 0);
}

#[tokio::test]
async fn test_delay() {
    let frequency = 4.194304 * 1e6;
    let mut clock = SystemClock::from_frequency(frequency);
    clock.next().await;
    tokio::time::delay_for(tokio::time::Duration::from_nanos(500)).await;
    clock.next().await;
    assert!(clock.avg_delay() > 0);
}
