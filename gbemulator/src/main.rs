mod system;
use system::ram::mapping_chip::{DynamicMappingChip, FakeChip};

use crate::system::ram::mapping_chip::{NoChip, MBC1};

#[show_image::main]
fn main() {
    let gameboy = system::System::new(
        Some(DynamicMappingChip::NoChip(NoChip::from_rom_path(
            "./ttr.gb",
        ))),
        false,
    );
    let n_cycles = 60 * 1_000_000;
    let _ = gameboy.run(n_cycles);
}
