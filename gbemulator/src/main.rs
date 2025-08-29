mod system;

#[show_image::main]
fn main() {
    println!("Hello, world!");
    let gameboy = system::System::new(1e5);
    let n_cycles = 1_000_000;
    let _ = gameboy.run(n_cycles);
}
