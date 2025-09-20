use std::{
    sync::{atomic::AtomicBool, Arc},
    time::{Duration, Instant},
};

fn period_to_duration(period: f32) -> Duration {
    let duration_nanos = (period * 1e9).ceil() as u64;
    return Duration::from_nanos(duration_nanos);
}

pub struct MasterClock {
    cpu_duration: Duration,
    lcd_duration: Duration,
    sound_duration: Duration,
    last_cpu: Instant,
    last_lcd: Instant,
    last_sound: Instant,
}

impl MasterClock {
    pub fn from_frequencies(cpu_frequency: f32, lcd_frequency: f32, sound_frequency: f32) -> Self {
        MasterClock {
            cpu_duration: period_to_duration(1. / cpu_frequency),
            lcd_duration: period_to_duration(1. / lcd_frequency),
            sound_duration: period_to_duration(1. / sound_frequency),
            last_cpu: Instant::now(),
            last_lcd: Instant::now(),
            last_sound: Instant::now(),
        }
    }

    pub fn new() -> Self {
        Self::from_frequencies(1e5, 1e6 / 19.0f32, 1e3)
    }

    pub fn start(&mut self) {
        self.last_cpu = Instant::now();
        self.last_lcd = Instant::now();
        self.last_sound = Instant::now();
    }

    pub fn next(
        &mut self,
        cpu_ready: &Arc<AtomicBool>,
        lcd_ready: &Arc<AtomicBool>,
        sound_ready: &Arc<AtomicBool>,
    ) {
        if self.last_cpu.elapsed() > self.cpu_duration {
            cpu_ready.store(true, std::sync::atomic::Ordering::Relaxed);
            self.last_cpu = Instant::now();
        }

        if self.last_lcd.elapsed() > self.lcd_duration {
            lcd_ready.store(true, std::sync::atomic::Ordering::Relaxed);
            self.last_lcd = Instant::now();
        }

        if self.last_sound.elapsed() > self.sound_duration {
            sound_ready.store(true, std::sync::atomic::Ordering::Relaxed);
            self.last_sound = Instant::now();
        }
    }
}
