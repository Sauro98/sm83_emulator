pub struct RAM {
    data: [u8; 65535],
}

impl RAM {
    pub fn new() -> RAM {
        RAM { data: [0; 65535] }
    }

    pub fn get_at(&self, address: u16) -> Option<&u8> {
        self.data.get(address as usize)
    }

    pub fn set_at(&mut self, address: u16, value: u8) -> Option<()> {
        match self.data.get_mut(address as usize) {
            Some(x) => *x = value,
            None => return None,
        }
        Some(())
    }
}
