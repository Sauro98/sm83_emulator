use crate::system::ram::sound_registers::Channel4Registers;

use super::clock::SystemClock;
use super::ram::sound_registers::{
    Channel1Registers, Channel2Registers, Channel3Registers, SoundOutputSelectionRegister,
    SoundRegisters,
};
use super::ram::{MemoryRegister, RAM};
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::env;
use std::io::IntoInnerError;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
#[derive(Clone)]
pub struct ToneNSweep {
    last_period: Instant,
    sweep_time: Duration,
    last_sweep_time: Instant,
    sweep_direction: bool,
    sweep_shifts: u8,
    sweep_count: u8,
    wave_pattern_duty: u8,
    sound_length: Option<Duration>,
    initial_volume: u8,
    current_volume: u8,
    envelope_direction: bool,
    envelope_sweeps: u8,
    last_envelope_time: Instant,
    envelope_count: u8,
    initial_frequency: u32,
    current_frequency: u32,
    initial: bool,
    initial_time: Instant,
    counter_consecutive_selection: bool,
    phase: f32,
}

impl ToneNSweep {
    pub fn new(
        sweep_time: Duration,
        sweep_direction: bool,
        sweep_shifts: u8,
        wave_pattern_duty: u8,
        sound_length: Option<Duration>,
        initial_volume: u8,
        envelope_direction: bool,
        envelope_sweeps: u8,
        initial_frequency: u32,
        initial: bool,
        counter_consecutive_selection: bool,
    ) -> Self {
        ToneNSweep {
            last_period: Instant::now(),
            sweep_time,
            last_sweep_time: Instant::now(),
            sweep_direction,
            sweep_shifts,
            sweep_count: 0,
            wave_pattern_duty,
            sound_length,
            initial_volume,
            current_volume: initial_volume,
            envelope_direction,
            envelope_sweeps,
            last_envelope_time: Instant::now(),
            envelope_count: 0,
            initial_frequency,
            current_frequency: initial_frequency,
            initial,
            initial_time: Instant::now(),
            counter_consecutive_selection,
            phase: 0.0f32,
        }
    }

    pub fn from_channel1(channel1_registers: &mut Channel1Registers) -> Self {
        let sweep_register_value = channel1_registers.sweep.get_value();
        let sweep_time = match ((sweep_register_value & 0x70) >> 4) {
            0x0 => Duration::ZERO,
            0x1 => Duration::from_micros(7800),
            0x2 => Duration::from_micros(15600),
            0x3 => Duration::from_micros(23400),
            0x4 => Duration::from_micros(31300),
            0x5 => Duration::from_micros(39100),
            0x6 => Duration::from_micros(46900),
            0x7 => Duration::from_micros(54700),
            _ => Duration::ZERO,
        };
        let sweep_direction = (sweep_register_value & 0x08) == 0;
        let sweep_shifts = sweep_register_value & 0x07;

        let wave_pattern_register_value = channel1_registers.wave_pattern.get_value();
        let wave_pattern_duty = (wave_pattern_register_value & 0xC0) >> 6;
        let sound_length = 64 - (wave_pattern_register_value & 0x3F);
        let sound_length = sound_length as u64 * 3906250u64;

        let volume_register_value = channel1_registers.volume_envelope.get_value();
        let initial_volume = (volume_register_value & 0xF0) >> 4;
        let envelope_direction = (volume_register_value & 0x80) > 0;
        let envelope_steps = volume_register_value & 0x03;

        let freq_hi_register_value = channel1_registers.frequency_hi.get_value();

        let initial_frequency_lower = channel1_registers.frequency_lo.get_value();
        let initial_frequency_upper = freq_hi_register_value & 0x07;
        let initial_frequency =
            ((initial_frequency_upper as u16) << 8) | initial_frequency_lower as u16;
        let initial_frequency_u16 = initial_frequency;

        let initial_frequency = 131072 / (2048 - initial_frequency as u32);

        let initial = (freq_hi_register_value & 0x80) > 0;
        // reset_initial flag
        if initial {
            channel1_registers
                .frequency_hi
                .set_value(freq_hi_register_value & 0x7F);
        }
        let counter_consecutive_selection = (freq_hi_register_value & 0x40) > 0;

        if initial {
            println!(
                "Initial frequency {}Hz, {}",
                initial_frequency, initial_frequency_u16
            );
            println!("Wave duty {}", wave_pattern_duty);
            println!("Counter selection {}", counter_consecutive_selection);
            println!("Sound duration {:?}", sound_length);
            println!("sweep_shifts {:?}", sweep_shifts);
            println!("sweep_direction {:?}", sweep_direction);
            println!("sweep_time {:?}", sweep_time);
            println!("initial volume {}", initial_volume);
            println!("initial {}", initial);
            println!("envelope sweeping {}", envelope_steps);
            println!("envelope direction {}", envelope_direction);
            println!("-------------------");
        }

        ToneNSweep {
            last_period: Instant::now(),
            sweep_time: sweep_time,
            last_sweep_time: Instant::now(),
            sweep_direction: sweep_direction,
            sweep_shifts: sweep_shifts,
            sweep_count: 0,
            wave_pattern_duty: wave_pattern_duty,
            sound_length: if counter_consecutive_selection {
                Some(Duration::from_nanos(sound_length))
            } else {
                None
            },
            initial_volume: initial_volume,
            current_volume: initial_volume,
            envelope_direction: envelope_direction,
            envelope_sweeps: envelope_steps,
            envelope_count: 0,
            last_envelope_time: Instant::now(),
            initial_frequency: initial_frequency,
            current_frequency: initial_frequency,
            initial: initial,
            initial_time: Instant::now(),
            counter_consecutive_selection: counter_consecutive_selection,
            phase: 0.0f32,
        }
    }

    pub fn from_channel2(channel2_registers: &mut Channel2Registers) -> Self {
        let wave_pattern_register_value = channel2_registers.wave_pattern.get_value();
        let wave_pattern_duty = (wave_pattern_register_value & 0xC0) >> 6;
        let sound_length = 64 - (wave_pattern_register_value & 0x3F);
        let sound_length = sound_length as u64 * 3906250u64;

        let volume_register_value = channel2_registers.volume_envelope.get_value();
        let initial_volume = (volume_register_value & 0xF0) >> 4;
        let envelope_direction = (volume_register_value & 0x80) > 0;
        let envelope_steps = volume_register_value & 0x07;

        let initial_frequency = channel2_registers.frequency_lo.get_value() as u16;

        let freq_hi_register_value = channel2_registers.frequency_hi.get_value();

        let initial_frequency =
            2048 - (initial_frequency | (((freq_hi_register_value & 0x03) as u16) << 8));
        let initial_frequency = 131072u32 / (initial_frequency as u32);

        let initial = (freq_hi_register_value & 0x80) > 0;
        // reset_initial flag
        if initial {
            channel2_registers
                .frequency_hi
                .set_value(freq_hi_register_value & 0x7F);
        }

        let counter_consecutive_selection = (freq_hi_register_value & 0x40) > 0;

        if initial {
            println!("Initial frequency {}Hz", initial_frequency);
            println!("Wave duty {}", wave_pattern_duty);
            println!("Counter selection {}", counter_consecutive_selection);
            println!("Sound duration {:?}", sound_length);
            println!("initial volume {}", initial_volume);
            println!("initial {}", initial);
            println!("envelope sweeping {}", envelope_steps);
            println!("envelope direction {}", envelope_direction);
            println!("-------------------");
        }

        ToneNSweep {
            last_period: Instant::now(),
            sweep_time: Duration::ZERO,
            last_sweep_time: Instant::now(),
            sweep_direction: false,
            sweep_shifts: 0,
            sweep_count: 0,
            wave_pattern_duty: wave_pattern_duty,
            sound_length: if counter_consecutive_selection {
                Some(Duration::from_nanos(sound_length))
            } else {
                None
            },
            initial_volume: initial_volume,
            current_volume: initial_volume,
            envelope_direction: envelope_direction,
            envelope_sweeps: envelope_steps,
            last_envelope_time: Instant::now(),
            envelope_count: 0,
            initial_frequency: initial_frequency,
            current_frequency: initial_frequency,
            initial: initial,
            initial_time: Instant::now(),
            counter_consecutive_selection: counter_consecutive_selection,
            phase: 0.0f32,
        }
    }

    pub fn period(&self) -> Duration {
        let period = 1e6 as u32 / self.current_frequency;
        Duration::from_micros(period as u64)
    }

    pub fn period_nanos(&self) -> u64 {
        1e9 as u64 / self.current_frequency as u64
    }

    pub fn wave_duty(&self) -> f32 {
        (match self.wave_pattern_duty {
            0 => 12.5,
            1 => 25.0,
            2 => 50.0,
            3 => 75.0,
            _ => 50.0,
        }) / 100.0
    }

    pub fn get_current_volume(&mut self) -> f32 {
        if self.envelope_count > self.envelope_sweeps {
            return self.current_volume as f32 / 15.0;
        }
        let envelope_duration = (self.envelope_sweeps as u64 * 1e6 as u64) / 64;
        let envelope_duration = Duration::from_micros(envelope_duration);
        if self.last_envelope_time.elapsed() > envelope_duration {
            self.last_envelope_time = Instant::now();
            if self.envelope_direction {
                if self.current_volume < 15 {
                    self.current_volume += 1;
                }
            } else {
                if self.current_volume > 0 {
                    self.current_volume -= 1;
                }
            }
            self.envelope_count += 1;
        }
        return self.current_volume as f32 / 15.0;
    }

    pub fn is_initial(&self) -> bool {
        self.initial
    }
}

impl Iterator for ToneNSweep {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(length) = self.sound_length {
            if self.initial_time.elapsed() > length {
                if !self.counter_consecutive_selection {
                    self.initial_time = Instant::now();
                    self.current_frequency = self.initial_frequency;
                    self.current_volume = self.initial_volume;
                    self.envelope_count = 0;
                    self.sweep_count = 0;
                    self.last_sweep_time = Instant::now();
                    self.last_envelope_time = Instant::now();
                    self.last_period = Instant::now();
                    self.sweep_shifts = 0;
                } else {
                    return None;
                }
            }
        }
        if self.sweep_shifts > 0 && self.last_sweep_time.elapsed() > self.sweep_time {
            let delta = self.current_frequency / 2u32.pow(self.sweep_shifts as u32);
            self.current_frequency = if self.sweep_direction {
                self.current_frequency + delta
            } else {
                self.current_frequency - delta
            };
            self.last_sweep_time = Instant::now();
            self.sweep_count += 1;
        }

        if self.sweep_count > self.sweep_shifts {
            if !self.counter_consecutive_selection {
                self.sweep_count = 0;
                self.current_frequency = self.initial_frequency;
                self.current_volume = self.initial_volume;
                self.envelope_count = 0;
                self.last_sweep_time = Instant::now();
                self.last_envelope_time = Instant::now();
            } else {
                return None;
            }
        }

        let res = if self.phase < self.wave_duty() {
            self.get_current_volume()
        } else {
            0.0f32
        };
        let phase_step = self.current_frequency as f32 / 48000.0;
        self.phase = (self.phase + phase_step).rem_euclid(1.0f32);
        Some(res)
    }
}

impl Source for ToneNSweep {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        48000
    }

    fn total_duration(&self) -> Option<Duration> {
        if self.counter_consecutive_selection {
            self.sound_length
        } else {
            None
        }
    }
}

pub struct SoundTerminal {
    volume: u8,
    sound_1: Option<ToneNSweep>,
    sound_2: Option<ToneNSweep>,
    sound_3: Option<ToneNSweep>,
    sound_4: Option<ToneNSweep>,
}

impl SoundTerminal {
    pub fn new() -> Self {
        SoundTerminal {
            volume: 0,
            sound_1: None,
            sound_2: None,
            sound_3: None,
            sound_4: None,
        }
    }

    pub fn set_volume(&mut self, volume: u8) {
        if volume != self.volume {
            println!(
                "Sound terminal volume changed from {} to {}",
                self.volume, volume
            );
        }
        self.volume = volume;
    }

    pub fn consume(&mut self) -> Vec<ToneNSweep> {
        let mut sounds = vec![];
        if let Some(s1) = &self.sound_1 {
            sounds.push(s1.clone());
        }
        if let Some(s2) = &self.sound_2 {
            sounds.push(s2.clone());
        }
        if let Some(s3) = &self.sound_3 {
            sounds.push(s3.clone());
        }
        if let Some(s4) = &self.sound_4 {
            sounds.push(s4.clone());
        }
        self.sound_1 = None;
        self.sound_2 = None;
        self.sound_3 = None;
        self.sound_4 = None;
        sounds
    }

    pub fn set_sound1(&mut self, has_sound: bool, channel_1_registers: &mut Channel1Registers) {
        if has_sound {
            let tmp_sound = Some(ToneNSweep::from_channel1(channel_1_registers)).unwrap();
            if tmp_sound.is_initial() {
                self.sound_1 = Some(tmp_sound);
                println!(
                    "Sound terminal sound 1 changed from {} to {}",
                    false, has_sound
                );
            }
        }
    }

    pub fn set_sound2(&mut self, has_sound: bool, channel_2_registers: &mut Channel2Registers) {
        if has_sound {
            let tmp_sound = Some(ToneNSweep::from_channel2(channel_2_registers)).unwrap();
            if tmp_sound.is_initial() {
                self.sound_2 = Some(tmp_sound);
                println!(
                    "Sound terminal sound 2 changed from {} to {}",
                    false, has_sound
                );
            }
        }
    }

    pub fn set_sound3(&mut self, has_sound: bool, channel_3_registers: &mut Channel3Registers) {
        if has_sound {
            if channel_3_registers.frequency_hi.get_value() & 0x80 > 0 {
                println!(
                    "Sound terminal sound 3 changed from {} to {}",
                    false, has_sound
                );
            }
        }
    }

    pub fn set_sound4(&mut self, has_sound: bool, channel_4_registers: &mut Channel4Registers) {
        if has_sound {
            if channel_4_registers.counter_consecutive.get_value() & 0x80 > 0 {
                println!(
                    "Sound terminal sound 4 changed from {} to {}",
                    false, has_sound
                );
            }
        }
    }
}

pub struct SoundController {
    clock: SystemClock,
    sound_registers: SoundRegisters,
    thread_finished: Arc<Mutex<bool>>,
    thread_handle: std::thread::JoinHandle<()>,
    sound_on: bool,
    terminal_1: Arc<Mutex<SoundTerminal>>,
    terminal_2: Arc<Mutex<SoundTerminal>>,
}

impl SoundController {
    pub fn new() -> Self {
        let thread_finished = std::sync::Arc::new(std::sync::Mutex::new(false));
        let terminal_1_ref = Arc::new(Mutex::new(SoundTerminal::new()));
        let terminal_2_ref = Arc::new(Mutex::new(SoundTerminal::new()));
        SoundController {
            clock: SystemClock::from_frequency(1e3),
            sound_registers: SoundRegisters::new(),
            thread_finished: thread_finished.clone(),
            sound_on: false,
            terminal_1: terminal_1_ref.clone(),
            terminal_2: terminal_2_ref.clone(),
            thread_handle: std::thread::spawn(move || {
                // _stream must live as long as the sink
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
                loop {
                    //println!("inside sound loop");
                    {
                        let mut terminal_1 = terminal_1_ref.lock().unwrap();
                        let t1_sounds = terminal_1.consume();
                        let t1_volume = terminal_1.volume;
                        if t1_sounds.len() > 0 {
                            println!("Terminal 1 has {} sounds to play", t1_sounds.len());
                            sink.clear();
                            sink.play();
                        }
                        for sound in t1_sounds {
                            sink.append(sound.amplify(t1_volume as f32));
                            println!("appended sound to terminal 1");
                        }
                    }

                    /*let t2_sounds = terminal_2_ref.lock().unwrap().consume();
                    let t2_volume = terminal_2_ref.lock().unwrap().volume;
                    for sound in t2_sounds {
                        sink.append(sound.amplify(t2_volume as f32));
                        println!("appended sound to terminal 2");
                    }*/

                    let finished = thread_finished.lock().unwrap();
                    if *finished {
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                println!("outside sound loop");
            }),
        }
    }

    pub fn next(&mut self, ram: &Arc<Mutex<RAM>>) {
        {
            let mut ram = ram.lock().unwrap();
            self.sound_registers.read_from_ram(&ram);
            if self.sound_registers.sound_on_off.is_sound_on() != self.sound_on {
                println!(
                    "Sound control switched from {} to {}",
                    self.sound_on,
                    self.sound_registers.sound_on_off.is_sound_on()
                );
                self.sound_on = self.sound_registers.sound_on_off.is_sound_on();
            }
            if !self.sound_on {
                self.sound_registers.reset();
            } else {
                {
                    let mut terminal1 = self.terminal_1.lock().unwrap();
                    terminal1.set_volume(self.sound_registers.channel_control.terminal_1_volume());
                    let enabled_sounds_terminal_1 = self
                        .sound_registers
                        .sound_output_selection
                        .terminal_1_sound();
                    //println!("Terminal 1 enabled sounds: {:?}", enabled_sounds_terminal_1);
                    terminal1.set_sound1(
                        enabled_sounds_terminal_1[0],
                        &mut self.sound_registers.channel_1,
                    );
                    terminal1.set_sound2(
                        enabled_sounds_terminal_1[1],
                        &mut self.sound_registers.channel_2,
                    );
                    terminal1.set_sound3(
                        enabled_sounds_terminal_1[2],
                        &mut self.sound_registers.channel_3,
                    );
                    terminal1.set_sound4(
                        enabled_sounds_terminal_1[3],
                        &mut self.sound_registers.channel_4,
                    );

                    /*let mut terminal2 = self.terminal_2.lock().unwrap();
                    terminal2.set_volume(self.sound_registers.channel_control.terminal_2_volume());
                    let enabled_sounds_terminal_2 = self
                        .sound_registers
                        .sound_output_selection
                        .terminal_2_sound();
                    //println!("Terminal 1 enabled sounds: {:?}", enabled_sounds_terminal_1);
                    terminal2.set_sound1(
                        enabled_sounds_terminal_2[0],
                        &mut self.sound_registers.channel_1,
                    );
                    terminal2.set_sound2(
                        enabled_sounds_terminal_2[1],
                        &mut self.sound_registers.channel_2,
                    );
                    terminal2.set_sound3(
                        enabled_sounds_terminal_2[2],
                        &mut self.sound_registers.channel_3,
                    );
                    terminal2.set_sound4(
                        enabled_sounds_terminal_2[3],
                        &mut self.sound_registers.channel_4,
                    );*/
                }
                self.sound_registers.load_in_ram(&mut ram);
            }
        }
        self.clock.next();
    }

    pub fn stop_sound_thread(&self) {
        let mut reference = self.thread_finished.lock().unwrap();
        *reference = true;
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
