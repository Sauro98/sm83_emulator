pub mod clock;
pub mod ram;
pub mod sm83;

pub struct System {
    cpu: sm83::SM83,
    ram: ram::RAM,
}

impl System {
    pub fn new(clock_frequency: f32) -> System {
        return System {
            cpu: sm83::SM83::new(clock_frequency),
            ram: ram::RAM::new(),
        };
    }

    pub async fn next(&mut self) {
        self.cpu.next(&mut self.ram).await;
    }

    pub fn cycle_count(&self) -> u128 {
        return self.cpu.cycle_count;
    }
}
