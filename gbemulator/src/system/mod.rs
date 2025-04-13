use sm83::snapshot::SM83Snapshot;

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

    pub fn from_ram_snapshot(
        clock_frequency: f32,
        ram: ram::RAM,
        snapshot: SM83Snapshot,
    ) -> System {
        let mut cpu = sm83::SM83::new(clock_frequency);
        cpu.load_snapshot(snapshot);
        cpu.fetch_cycle(&ram);
        return System { cpu: cpu, ram: ram };
    }

    pub async fn next(&mut self) {
        self.cpu.next(&mut self.ram).await;
    }

    pub fn cycle_count(&self) -> u128 {
        return self.cpu.cycle_count;
    }

    pub fn to_snapshot(&self) -> SM83Snapshot {
        return self.cpu.to_snapshot();
    }
}
