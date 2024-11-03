use gbemulator::system::System;

#[tokio::test]
async fn test_next() {
    let frequency = 4.194304;
    let mut system = System::new(frequency);
    system.next().await;
    system.next().await;
    assert_eq!(system.cycle_count(), 2);
}
