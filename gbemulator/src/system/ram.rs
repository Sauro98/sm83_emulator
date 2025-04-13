pub struct RAM {
    data: std::vec::Vec<u8>,
    capacity: usize,
}

impl RAM {
    pub fn new() -> RAM {
        let capacity = u16::MAX as usize + 1;
        let data = vec![0; capacity];
        RAM {
            data: data,
            capacity: capacity,
        }
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
