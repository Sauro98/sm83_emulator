pub struct IDU {}

impl IDU {
    pub fn increment(v: u16) -> u16 {
        if v == u16::MAX {
            0
        } else {
            v + 1
        }
    }

    pub fn decrement(v: u16) -> u16 {
        if v == 0 {
            u16::MAX
        } else {
            v - 1
        }
    }
}
