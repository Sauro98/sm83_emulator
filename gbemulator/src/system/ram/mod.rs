pub mod lcd_registers;
pub mod sound_registers;

const BOOT_ROM_END: u16 = 0x00FF;
const BOOTLOCKER_ADDRESS: u16 = 0xFF50;
const BOOTLOCKER_LOCKED: u8 = 0x01;
const BOOTLOCKER_UNLOCKED: u8 = 0x00;
const DMA_ADDRESS: u16 = 0xFF46;

macro_rules! default_memory_register_trait_impl {
    ($name:ident,$reset_value:expr) => {
        impl MemoryRegister for $name {
            fn get_address(&self) -> u16 {
                self.address
            }

            fn get_value(&self) -> u8 {
                self.value
            }

            fn set_value(&mut self, value: u8) {
                self.value = value;
            }

            fn reset(&mut self) {
                self.value = $reset_value;
            }
        }
    };
}
pub(crate) use default_memory_register_trait_impl;

macro_rules! default_nonimplemented_memory_register_trait_impl {
    () => {
        fn get_address(&self) -> u16 {
            unimplemented!()
        }

        fn get_value(&self) -> u8 {
            unimplemented!()
        }

        fn set_value(&mut self, _: u8) {
            unimplemented!()
        }
    };
}
pub(crate) use default_nonimplemented_memory_register_trait_impl;

pub struct RAM {
    data: std::vec::Vec<u8>,
    #[allow(dead_code)]
    capacity: usize,
    dma_requested: bool,
}

impl Clone for RAM {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            capacity: self.capacity.clone(),
            dma_requested: self.dma_requested.clone(),
        }
    }
}

impl RAM {
    pub fn new() -> RAM {
        let capacity = u16::MAX as usize + 1;
        let data = vec![0; capacity];
        RAM {
            data: data,
            capacity: capacity,
            dma_requested: false,
        }
    }

    pub fn get_at(&self, address: u16) -> Option<u8> {
        self.data.get(address as usize).copied()
    }

    pub fn set_at(&mut self, address: u16, value: u8) -> Option<()> {
        if self.data.get(BOOTLOCKER_ADDRESS as usize) == Some(&BOOTLOCKER_LOCKED) {
            // if bootlock is set
            if address <= BOOT_ROM_END {
                return None; // cannot set boot memory if lock is on
            }
        }
        if address == DMA_ADDRESS {
            self.dma_requested = true;
        }
        match self.data.get_mut(address as usize) {
            Some(x) => *x = value,
            None => return None,
        }
        Some(())
    }

    pub fn was_dma_requested(&self) -> bool {
        self.dma_requested
    }

    pub fn reset_dma_request(&mut self) {
        self.dma_requested = false;
    }

    pub fn get_tile_data(
        &self,
        start_address: u16,
        offset: u8,
        is_offset_negative: bool,
    ) -> [u8; 16] {
        let mut result = [0; 16];
        let mut adjusted_offset = (start_address + (offset as u16) * 16) as usize;
        if is_offset_negative {
            let negative_offset = ((offset as i8) as i32 + 128) * 16;
            adjusted_offset = ((start_address as i32 + negative_offset) as u16) as usize;
        }

        for i in 0..16 {
            result[i] = *self.data.get(adjusted_offset + i).unwrap();
        }
        result
    }
}

pub trait MemoryRegister {
    fn get_address(&self) -> u16;
    fn get_value(&self) -> u8;
    fn set_value(&mut self, value: u8);
    fn reset(&mut self) {
        self.set_value(0x00);
    }
    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        ram.set_at(self.get_address(), self.get_value())
    }
    fn read_from_ram(&mut self, ram: &RAM) {
        self.set_value(ram.get_at(self.get_address()).unwrap());
    }
}

pub struct BootLockMemoryRegister {
    address: u16,
    value: u8,
}

pub struct BootRom {
    contents: Vec<u8>,
}

impl BootLockMemoryRegister {
    pub fn new() -> Self {
        BootLockMemoryRegister {
            address: BOOTLOCKER_ADDRESS,
            value: BOOTLOCKER_UNLOCKED,
        }
    }

    pub fn lock(&mut self) {
        self.value = BOOTLOCKER_LOCKED;
    }

    pub fn unlock(&mut self) {
        self.value = BOOTLOCKER_UNLOCKED;
    }

    pub fn is_unlocked(&self) -> bool {
        self.value == BOOTLOCKER_UNLOCKED
    }
}

default_memory_register_trait_impl!(BootLockMemoryRegister, BOOTLOCKER_UNLOCKED);

impl BootRom {
    pub fn new() -> Self {
        BootRom {
            contents: vec![
                0x31, 0xfe, 0xff, 0xaf, 0x21, 0xff, 0x9f, 0x32, 0xcb, 0x7c, 0x20, 0xfb, 0x21, 0x26,
                0xff, 0x0e, 0x11, 0x3e, 0x80, 0x32, 0xe2, 0x0c, 0x3e, 0xf3, 0xe2, 0x32, 0x3e, 0x77,
                0x77, 0x3e, 0xfc, 0xe0, 0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1a, 0xcd, 0x95,
                0x00, 0xcd, 0x96, 0x00, 0x13, 0x7b, 0xfe, 0x34, 0x20, 0xf3, 0x11, 0xd8, 0x00, 0x06,
                0x08, 0x1a, 0x13, 0x22, 0x23, 0x05, 0x20, 0xf9, 0x3e, 0x19, 0xea, 0x10, 0x99, 0x21,
                0x2f, 0x99, 0x0e, 0x0c, 0x3d, 0x28, 0x08, 0x32, 0x0d, 0x20, 0xf9, 0x2e, 0x0f, 0x18,
                0xf3, 0x67, 0x3e, 0x64, 0x57, 0xe0, 0x42, 0x3e, 0x91, 0xe0, 0x40, 0x04, 0x1e, 0x02,
                0x0e, 0x0c, 0xf0, 0x44, 0xfe, 0x90, 0x20, 0xfa, 0x0d, 0x20, 0xf7, 0x1d, 0x20, 0xf2,
                0x0e, 0x13, 0x24, 0x7c, 0x1e, 0x83, 0xfe, 0x62, 0x28, 0x06, 0x1e, 0xc1, 0xfe, 0x64,
                0x20, 0x06, 0x7b, 0xe2, 0x0c, 0x3e, 0x87, 0xe2, 0xf0, 0x42, 0x90, 0xe0, 0x42, 0x15,
                0x20, 0xd2, 0x05, 0x20, 0x4f, 0x16, 0x20, 0x18, 0xcb, 0x4f, 0x06, 0x04, 0xc5, 0xcb,
                0x11, 0x17, 0xc1, 0xcb, 0x11, 0x17, 0x05, 0x20, 0xf5, 0x22, 0x23, 0x22, 0x23, 0xc9,
                0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c,
                0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6,
                0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc,
                0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e, 0x3c, 0x42, 0xb9, 0xa5, 0xb9, 0xa5, 0x42, 0x3c,
                0x21, 0x04, 0x01, 0x11, 0xa8, 0x00, 0x1a, 0x13, 0xbe, 0x20, 0xfe, 0x23, 0x7d, 0xfe,
                0x34, 0x20, 0xf5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xfb, 0x86, 0x20, 0xfe,
                0x3e, 0x01, 0xe0, 0x50,
            ],
        }
    }
}

impl MemoryRegister for BootRom {
    fn reset(&mut self) {
        self.contents = vec![0x00; 255];
    }

    default_nonimplemented_memory_register_trait_impl!();

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        for (address, value) in self.contents.iter().enumerate() {
            let res = ram.set_at(address as u16, value.to_owned());
            if res.is_none() {
                return res;
            }
        }
        Some(())
    }

    fn read_from_ram(&mut self, _: &RAM) {
        unimplemented!()
    }
}

pub struct FakeRom {
    address: u16,
    contents: Vec<u8>,
}

impl FakeRom {
    pub fn new() -> Self {
        FakeRom {
            address: 0x0104,
            contents: vec![
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
        }
    }
}

impl MemoryRegister for FakeRom {
    fn reset(&mut self) {
        unimplemented!()
    }

    fn load_in_ram(&self, ram: &mut RAM) -> Option<()> {
        for i in 0..self.contents.len() {
            ram.set_at(self.address + i as u16, self.contents[i]);
        }
        Some(())
    }

    default_nonimplemented_memory_register_trait_impl!();

    fn read_from_ram(&mut self, _: &RAM) {
        unimplemented!()
    }
}

pub struct FakeChecksum {
    address: u16,
    value: u8,
}

impl FakeChecksum {
    pub fn new() -> Self {
        FakeChecksum {
            address: 0x014d,
            value: 0xE7,
        }
    }
}

default_memory_register_trait_impl!(FakeChecksum, 0xE7);
