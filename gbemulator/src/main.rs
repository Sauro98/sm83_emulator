use std::str::FromStr;

mod system;
use system::ram::mapping_chip::FakeChip;

#[show_image::main]
fn main() {
    let gameboy = system::System::new(1e5, None);
    let n_cycles = 200_000;
    let _ = gameboy.run(n_cycles);
}
