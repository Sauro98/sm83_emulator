mod system;

#[show_image::main]
fn main() {
    println!("Hello, world!");
    let gameboy = system::System::new(1e6);
    let n_cycles = 10_000_000;
    let _ = gameboy.run(n_cycles);
}
