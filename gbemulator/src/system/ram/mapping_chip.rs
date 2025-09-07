pub struct MemoryBank {
    pub address: u16,
    pub contents: [u8; 16 * 1024],
}

impl MemoryBank {
    pub fn new(address: u16) -> Self {
        MemoryBank {
            address: address,
            contents: [0u8; 16 * 1024],
        }
    }
}

pub trait MappingChip: Clone + Copy {
    fn is_selecting_chip_rom(&mut self, address: u16, value: u8) -> bool;
    fn is_selecting_chip_ram(&mut self, address: u16, value: u8) -> bool;
    fn get_selected_rom_bank(&self) -> MemoryBank;
    fn get_base_rom_bank(&self) -> MemoryBank;
    fn get_selected_ram_bank(&self) -> MemoryBank;
    fn is_ram_enabled(&self) -> bool;
    fn is_selecting_mode(&mut self, address: u16, value: u8) -> bool;
    fn new() -> Self;
}

#[derive(Clone, Copy)]
pub struct FakeChip {}

impl MappingChip for FakeChip {
    fn is_selecting_chip_rom(&mut self, _address: u16, _value: u8) -> bool {
        false
    }

    fn is_selecting_chip_ram(&mut self, _address: u16, _value: u8) -> bool {
        false
    }

    fn get_selected_rom_bank(&self) -> MemoryBank {
        let mut bank = MemoryBank::new(0x0000);

        let start_address = 0x0104 as usize;

        let contents = vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
            0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
            0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
            0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ];
        for i in 0..contents.len() {
            bank.contents[start_address + i] = contents[i];
        }
        bank.contents[0x014d as usize] = 0xE7;
        bank.contents[0x0100 as usize] = 0x00;
        bank.contents[0x0101 as usize] = 0xC3;
        bank.contents[0x0102 as usize] = 0x50;
        bank.contents[0x0103 as usize] = 0x01;
        bank
    }

    fn get_base_rom_bank(&self) -> MemoryBank {
        self.get_selected_rom_bank()
    }

    fn get_selected_ram_bank(&self) -> MemoryBank {
        MemoryBank::new(0x0000)
    }

    fn is_ram_enabled(&self) -> bool {
        false
    }

    fn is_selecting_mode(&mut self, _address: u16, _value: u8) -> bool {
        false
    }

    fn new() -> Self {
        FakeChip {}
    }
}

#[derive(Clone, Copy)]
pub enum MemoryMode {
    ROM,
    RAM,
}

#[derive(Clone, Copy)]
pub struct MBC1 {
    rom_bank: u8,
    ram_bank: u8,
    ram_enabled: bool,
    mode: MemoryMode,
}

impl MBC1 {
    const RAM_ENABLE_RANGE_START: u16 = 0x0000;
    const RAM_ENABLE_RANGE_END: u16 = 0x1FFF;
    const BANK1_RANGE_START: u16 = 0x2000;
    const BANK1_RANGE_END: u16 = 0x3FFF;
    const BANK2_RANGE_START: u16 = 0x4000;
    const BANK2_RANGE_END: u16 = 0x5FFF;
    const MODE_RANGE_START: u16 = 0x6000;
    const MODE_RANGE_END: u16 = 0x7FFF;
}

impl MappingChip for MBC1 {
    fn is_selecting_chip_rom(&mut self, address: u16, value: u8) -> bool {
        if address >= MBC1::BANK1_RANGE_START && address <= MBC1::BANK1_RANGE_END {
            let mut bank = value & 0x1F;
            if bank == 0 {
                bank = 1;
            }
            self.rom_bank |= bank;
            return true;
        }
        match self.mode {
            MemoryMode::ROM => {
                if address >= MBC1::BANK2_RANGE_START && address <= MBC1::BANK2_RANGE_END {
                    let bank = (value & 0x03) << 5;
                    self.rom_bank = (self.rom_bank & 0x1F) | bank;
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    fn is_selecting_mode(&mut self, address: u16, value: u8) -> bool {
        if address >= MBC1::MODE_RANGE_START && address <= MBC1::MODE_RANGE_END {
            self.mode = if value & 0x01 == 0 {
                MemoryMode::ROM
            } else {
                MemoryMode::RAM
            };
            return true;
        }
        false
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn is_selecting_chip_ram(&mut self, address: u16, value: u8) -> bool {
        if self.ram_enabled {
            if address >= MBC1::BANK2_RANGE_START && address <= MBC1::BANK2_RANGE_END {
                match self.mode {
                    MemoryMode::RAM => {
                        let bank = value & 0x03;
                        self.ram_bank = bank;
                        return true;
                    }
                    _ => {}
                }
            }
        }
        return false;
    }

    fn get_base_rom_bank(&self) -> MemoryBank {
        MemoryBank::new(0x4000)
    }

    fn get_selected_ram_bank(&self) -> MemoryBank {
        MemoryBank::new(0x4000)
    }

    fn get_selected_rom_bank(&self) -> MemoryBank {
        MemoryBank::new(0xA000)
    }

    fn new() -> Self {
        MBC1 {
            rom_bank: 0,
            ram_bank: 0,
            ram_enabled: false,
            mode: MemoryMode::ROM,
        }
    }
}

#[derive(Copy, Clone)]
pub enum DynamicMappingChip {
    FakeChip(FakeChip),
    MBC1(MBC1),
}

impl MappingChip for DynamicMappingChip {
    fn is_selecting_chip_rom(&mut self, address: u16, value: u8) -> bool {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.is_selecting_chip_rom(address, value),
            DynamicMappingChip::MBC1(mbc1) => mbc1.is_selecting_chip_rom(address, value),
        }
    }

    fn is_selecting_chip_ram(&mut self, address: u16, value: u8) -> bool {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.is_selecting_chip_ram(address, value),
            DynamicMappingChip::MBC1(mbc1) => mbc1.is_selecting_chip_ram(address, value),
        }
    }

    fn get_base_rom_bank(&self) -> MemoryBank {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.get_base_rom_bank(),
            DynamicMappingChip::MBC1(mbc1) => mbc1.get_base_rom_bank(),
        }
    }

    fn get_selected_rom_bank(&self) -> MemoryBank {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.get_selected_rom_bank(),
            DynamicMappingChip::MBC1(mbc1) => mbc1.get_selected_rom_bank(),
        }
    }

    fn get_selected_ram_bank(&self) -> MemoryBank {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.get_selected_ram_bank(),
            DynamicMappingChip::MBC1(mbc1) => mbc1.get_selected_ram_bank(),
        }
    }

    fn is_ram_enabled(&self) -> bool {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.is_ram_enabled(),
            DynamicMappingChip::MBC1(mbc1) => mbc1.is_ram_enabled(),
        }
    }

    fn is_selecting_mode(&mut self, address: u16, value: u8) -> bool {
        match self {
            DynamicMappingChip::FakeChip(fc) => fc.is_selecting_mode(address, value),
            DynamicMappingChip::MBC1(mbc1) => mbc1.is_selecting_mode(address, value),
        }
    }

    fn new() -> Self {
        DynamicMappingChip::FakeChip(FakeChip::new())
    }
}
