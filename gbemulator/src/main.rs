use gbemulator::system::ram;

mod system;

fn format_frequency(frequency: f32) -> String {
    if frequency < 1e3 {
        return format!("{:.2} Hz", frequency);
    } else if frequency < 1e6 {
        return format!("{:.2} kHz", frequency / 1e3);
    } else if frequency < 1e9 {
        return format!("{:.2} MHz", frequency / 1e6);
    } else if frequency < 1e12 {
        return format!("{:.2} GHz", frequency / 1e9);
    } else {
        return "Impossible frequency".to_string();
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let mut cpu = system::sm83::SM83::new(1_000_000.0);
    let mut ram = system::ram::RAM::new();
    for _ in 0..100_000 {
        cpu.next(&mut ram).await;
    }
    println!("{}", format_frequency(cpu.fps()));
    println!("{}", cpu.avg_delay());
}
