use crate::system::ram::sound_registers::Channel4Registers;

use crate::system::ram::sound_registers::{
    Channel1Registers, Channel2Registers, Channel3Registers, SoundOutputSelectionRegister,
    SoundRegisters,
};
use crate::system::ram::{MemoryRegister, RAM};
use rodio::source::Source;
use rodio::{OutputStream, Sink};
use std::path::Iter;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct VolumeEnvelope {
    initial_volume: f32,
    current_volume: f32,
    envelope_direction: f32,
    samples_per_sweep: u32,
    sample_count: u32,
}

impl VolumeEnvelope {
    pub fn new(
        initial_volume: f32,
        envelope_direction: bool,
        envelope_sweeps: u8,
        sample_rate: u32,
    ) -> Self {
        let envelope_duration = (envelope_sweeps as u64 * 1e6 as u64) / 64;
        let envelope_duration = Duration::from_micros(envelope_duration);
        let samples_per_sweep = envelope_duration.as_secs_f32() * sample_rate as f32;

        VolumeEnvelope {
            initial_volume,
            current_volume: initial_volume,
            envelope_direction: if envelope_direction { 1.0 } else { -1.0 },
            samples_per_sweep: samples_per_sweep as u32,
            sample_count: 0,
        }
    }

    pub fn get_current_volume(&mut self) -> f32 {
        return self.current_volume as f32 / 15.0;
    }

    pub fn reset(&mut self) {
        self.current_volume = self.initial_volume;
        self.sample_count = 0;
    }
}

impl Iterator for VolumeEnvelope {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_count >= self.samples_per_sweep as u32 {
            self.sample_count = 0;
            self.current_volume += self.envelope_direction;
            self.current_volume = self.current_volume.clamp(0.0f32, 15.0f32);
        }
        self.sample_count += 1;
        Some(self.get_current_volume())
    }
}

#[derive(Clone)]
pub struct FrequencyEnvelope {
    initial_frequency: u32,
    current_frequency: u32,
    sweep_direction: bool,
    sweep_shifts: u8,
    sweep_count: u8,
    samples_per_sweep: u32,
    sample_count: u32,
}

impl FrequencyEnvelope {
    pub fn new(
        initial_frequency: u32,
        sweep_duration: Duration,
        sweep_direction: bool,
        sweep_shifts: u8,
        sample_rate: u32,
    ) -> Self {
        let samples_per_sweep = sweep_duration.as_secs_f32() * sample_rate as f32;
        FrequencyEnvelope {
            initial_frequency,
            current_frequency: initial_frequency,
            sweep_direction,
            sweep_shifts,
            sweep_count: 0,
            samples_per_sweep: samples_per_sweep as u32,
            sample_count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.sweep_count = 0;
        self.current_frequency = self.initial_frequency;
        self.sample_count = 0;
    }
}

impl Iterator for FrequencyEnvelope {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sweep_shifts > 0 && self.sample_count >= self.samples_per_sweep {
            let delta = self.current_frequency / 2u32.pow(self.sweep_shifts as u32);
            self.current_frequency = if self.sweep_direction {
                self.current_frequency + delta
            } else {
                self.current_frequency - delta
            };
            self.sweep_count += 1;
        }
        self.sample_count += 1;
        Some(self.current_frequency as f32)
    }
}

#[derive(Clone)]
pub struct ToneNSweep {
    wave_pattern_duty: u8,
    total_sound_samples: Option<u32>,
    volume_envelope: VolumeEnvelope,
    frequency_envelope: FrequencyEnvelope,
    initial: bool,
    current_sample: u32,
    phase: f32,
}

impl ToneNSweep {
    pub const SAMPLE_RATE: f32 = 48000.0f32;
    pub fn new(
        sweep_time: Duration,
        sweep_direction: bool,
        sweep_shifts: u8,
        wave_pattern_duty: u8,
        sound_length: Option<Duration>,
        initial_volume: f32,
        envelope_direction: bool,
        envelope_sweeps: u8,
        initial_frequency: u32,
        initial: bool,
    ) -> Self {
        let total_sound_samples = if let Some(sl) = sound_length {
            Some((sl.as_secs_f32() * Self::SAMPLE_RATE) as u32)
        } else {
            None
        };
        ToneNSweep {
            wave_pattern_duty,
            total_sound_samples,
            volume_envelope: VolumeEnvelope::new(
                initial_volume,
                envelope_direction,
                envelope_sweeps,
                Self::SAMPLE_RATE as u32,
            ),
            frequency_envelope: FrequencyEnvelope::new(
                initial_frequency,
                sweep_time,
                sweep_direction,
                sweep_shifts,
                Self::SAMPLE_RATE as u32,
            ),
            initial,
            current_sample: 0,
            phase: 0.0f32,
        }
    }

    pub fn from_channel1(channel1_registers: &mut Channel1Registers) -> Self {
        let sweep_register_value = channel1_registers.sweep.get_value();
        let sweep_time = match (sweep_register_value & 0x70) >> 4 {
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
        let envelope_direction = (volume_register_value & 0x08) > 0;
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
        let repeat = !counter_consecutive_selection;

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
            wave_pattern_duty: wave_pattern_duty,
            total_sound_samples: if repeat {
                None
            } else {
                Some((Duration::from_nanos(sound_length).as_secs_f32() * Self::SAMPLE_RATE) as u32)
            },
            volume_envelope: VolumeEnvelope::new(
                initial_volume as f32,
                envelope_direction,
                envelope_steps,
                Self::SAMPLE_RATE as u32,
            ),
            frequency_envelope: FrequencyEnvelope::new(
                initial_frequency,
                sweep_time,
                sweep_direction,
                sweep_shifts,
                Self::SAMPLE_RATE as u32,
            ),
            initial: initial,
            current_sample: 0,
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
        let envelope_direction = (volume_register_value & 0x08) > 0;
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
        let repeat = !counter_consecutive_selection;

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
            wave_pattern_duty: wave_pattern_duty,
            total_sound_samples: if repeat {
                None
            } else {
                Some((Duration::from_nanos(sound_length).as_secs_f32() * Self::SAMPLE_RATE) as u32)
            },
            volume_envelope: VolumeEnvelope::new(
                initial_volume as f32,
                envelope_direction,
                envelope_steps,
                Self::SAMPLE_RATE as u32,
            ),
            frequency_envelope: FrequencyEnvelope::new(
                initial_frequency,
                Duration::ZERO,
                false,
                0,
                Self::SAMPLE_RATE as u32,
            ),
            initial: initial,
            current_sample: 0,
            phase: 0.0f32,
        }
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

    pub fn is_initial(&self) -> bool {
        self.initial
    }
}

impl Iterator for ToneNSweep {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(total_samples) = self.total_sound_samples {
            if self.current_sample >= total_samples {
                return None;
            }
        }
        self.current_sample += 1;
        let up_volume = self.volume_envelope.next().unwrap_or(0.0f32);
        let current_frequency = self.frequency_envelope.next().unwrap_or(0.0f32);
        let res = if self.phase < self.wave_duty() {
            up_volume
        } else {
            0.0f32
        };
        let phase_step = current_frequency / Self::SAMPLE_RATE;
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
        Self::SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        if let Some(total_samples) = self.total_sound_samples {
            Some(Duration::from_secs_f32(
                total_samples as f32 / Self::SAMPLE_RATE,
            ))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct WhiteNoise {
    total_sound_sampes: Option<u32>,
    current_sample: u32,
    volume_envelope: VolumeEnvelope,
    half_width: bool,
    lfsr: u16,
    initial: bool,
    phase: f32,
    frequency: f32,
}

impl WhiteNoise {
    pub const SAMPLE_RATE: f32 = 48000.0f32;

    pub fn new(
        sound_length: Option<Duration>,
        initial_volume: f32,
        envelope_direction: bool,
        envelope_sweeps: u8,
        half_width: bool,
        frequency: f32,
        initial: bool,
    ) -> Self {
        WhiteNoise {
            total_sound_sampes: if let Some(sl) = sound_length {
                Some((sl.as_secs_f32() * Self::SAMPLE_RATE) as u32)
            } else {
                None
            },
            current_sample: 0,
            volume_envelope: VolumeEnvelope::new(
                initial_volume,
                envelope_direction,
                envelope_sweeps,
                Self::SAMPLE_RATE as u32,
            ),
            half_width,
            lfsr: 0x0,
            initial,
            phase: 0.0f32,
            frequency,
        }
    }
    pub fn from_channel4(channel_4_registers: &mut Channel4Registers) -> Self {
        let sound_length_register_value = channel_4_registers.sound_length.get_value();
        let sound_length = 64 - (sound_length_register_value & 0x3F);
        let sound_length = sound_length as u64 * 3906250u64;

        let volume_register_value = channel_4_registers.volume_envelope.get_value();
        let initial_volume = (volume_register_value & 0xF0) >> 4;
        let envelope_direction = (volume_register_value & 0x08) > 0;
        let envelope_steps = volume_register_value & 0x07;

        let polynomial_counter_register_value = channel_4_registers.polynomial_counter.get_value();
        let shift_clock_frequency = (polynomial_counter_register_value & 0xF0) >> 4;
        let half_width = (polynomial_counter_register_value & 0x08) > 0;
        let dividing_ratio = polynomial_counter_register_value & 0x07;
        let dividing_ratio = if dividing_ratio == 0 {
            0.5f32
        } else {
            dividing_ratio as f32
        };
        let frequency = 524288f32 / dividing_ratio / 2f32.powi(shift_clock_frequency as i32 + 1);

        let counter_consecutive_register_value =
            channel_4_registers.counter_consecutive.get_value();
        let initial = (counter_consecutive_register_value & 0x80) > 0;
        let counter_consecutive_selection = (counter_consecutive_register_value & 0x40) > 0;
        let repeat = !counter_consecutive_selection;

        // reset_initial flag
        if initial {
            channel_4_registers
                .counter_consecutive
                .set_value(counter_consecutive_register_value & 0x7F);
        }

        if initial {
            println!("Initial frequency {}Hz", frequency);
            println!("Counter selection {}", counter_consecutive_selection);
            println!("Sound duration {:?}", sound_length);
            println!("initial volume {}", initial_volume);
            println!("initial {}", initial);
            println!("envelope sweeping {}", envelope_steps);
            println!("envelope direction {}", envelope_direction);
            println!("half width {}", half_width);
            println!("-------------------");
        }

        WhiteNoise {
            total_sound_sampes: if repeat {
                None
            } else {
                Some((Duration::from_nanos(sound_length).as_secs_f32() * Self::SAMPLE_RATE) as u32)
            },
            current_sample: 0,
            volume_envelope: VolumeEnvelope::new(
                initial_volume as f32,
                envelope_direction,
                envelope_steps,
                Self::SAMPLE_RATE as u32,
            ),
            half_width: half_width,
            lfsr: 0,
            initial: initial,
            phase: 0.0f32,
            frequency: frequency,
        }
    }

    fn is_initial(&self) -> bool {
        self.initial
    }
}

impl Iterator for WhiteNoise {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(total_samples) = self.total_sound_sampes {
            if self.current_sample >= total_samples {
                return None;
            }
        }
        self.current_sample += 1;

        let up_volume = self.volume_envelope.next().unwrap_or(0.0f32);

        let phase_step = self.frequency as f32 / Self::SAMPLE_RATE;
        self.phase = self.phase + phase_step;
        if self.phase >= 1.0f32 {
            let comparison = ((self.lfsr & 0x0002) >> 1) == (self.lfsr & 0x0001);
            let mut addend = (comparison as u16) << 15;
            if self.half_width {
                self.lfsr &= 0xFF7F;
                addend |= (comparison as u16) << 7;
            }
            self.lfsr |= addend;
            self.lfsr >>= 1;
        }
        self.phase = self.phase.rem_euclid(1.0f32);
        Some((self.lfsr & 0x0001) as f32 * up_volume)
    }
}

impl Source for WhiteNoise {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        Self::SAMPLE_RATE as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        if let Some(total_samples) = self.total_sound_sampes {
            Some(Duration::from_secs_f32(
                total_samples as f32 / Self::SAMPLE_RATE,
            ))
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
    sound_4: Option<WhiteNoise>,
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

    pub fn consume_sound1(&mut self) -> Option<ToneNSweep> {
        let sound = self.sound_1.clone();
        self.sound_1 = None;
        sound
    }
    pub fn consume_sound2(&mut self) -> Option<ToneNSweep> {
        let sound = self.sound_2.clone();
        self.sound_2 = None;
        sound
    }
    pub fn consume_sound3(&mut self) -> Option<ToneNSweep> {
        let sound = self.sound_3.clone();
        self.sound_3 = None;
        sound
    }
    pub fn consume_sound4(&mut self) -> Option<WhiteNoise> {
        let sound = self.sound_4.clone();
        self.sound_4 = None;
        sound
    }

    pub fn set_sounds(&mut self, has_sound: [bool; 4], sound_regsters: &mut SoundRegisters) {
        self.set_sound1(has_sound[0], &mut sound_regsters.channel_1);
        self.set_sound2(has_sound[1], &mut sound_regsters.channel_2);
        self.set_sound3(has_sound[2], &mut sound_regsters.channel_3);
        self.set_sound4(has_sound[3], &mut sound_regsters.channel_4);
    }

    pub fn set_sound1(&mut self, has_sound: bool, channel_1_registers: &mut Channel1Registers) {
        if has_sound {
            let tmp_sound = Some(ToneNSweep::from_channel1(channel_1_registers));
            if tmp_sound.as_ref().unwrap().is_initial() {
                self.sound_1 = tmp_sound;
                println!(
                    "Sound terminal sound 1 changed from {} to {}",
                    false, has_sound
                );
            }
        }
    }

    pub fn set_sound2(&mut self, has_sound: bool, channel_2_registers: &mut Channel2Registers) {
        if has_sound {
            let tmp_sound = Some(ToneNSweep::from_channel2(channel_2_registers));
            if tmp_sound.as_ref().unwrap().is_initial() {
                self.sound_2 = tmp_sound;
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
            let tmp_sound = Some(WhiteNoise::from_channel4(channel_4_registers));
            if tmp_sound.as_ref().unwrap().is_initial() {
                println!(
                    "Sound terminal sound 4 changed from {} to {}",
                    false, has_sound
                );
                self.sound_4 = tmp_sound;
            }
        }
    }
}

pub fn reproduce_sound_terminal(
    terminal_ref: Arc<Mutex<SoundTerminal>>,
    sinks: &Vec<Sink>,
    terminal_name: &str,
) {
    let mut terminal = terminal_ref.lock().unwrap();
    let volume = terminal.volume;

    if let Some(sound1) = terminal.consume_sound1() {
        println!("Consuming sound 1 in terminal {}", terminal_name);
        sinks[0].clear();
        sinks[0].append(sound1.amplify(volume as f32));
        sinks[0].play();
    }

    if let Some(sound2) = terminal.consume_sound2() {
        println!("Consuming sound 2 in terminal {}", terminal_name);
        sinks[1].clear();
        sinks[1].append(sound2.amplify(volume as f32));
        sinks[1].play();
    }

    if let Some(sound3) = terminal.consume_sound3() {
        println!("Consuming sound 1 in terminal {}", terminal_name);
        sinks[2].clear();
        sinks[2].append(sound3.amplify(volume as f32));
        sinks[2].play();
    }

    if let Some(sound4) = terminal.consume_sound4() {
        println!("Consuming sound 1 in terminal {}", terminal_name);
        sinks[3].clear();
        sinks[3].append(sound4.amplify(volume as f32));
        sinks[3].play();
    }
}

pub struct SoundController {
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
            sound_registers: SoundRegisters::new(),
            thread_finished: thread_finished.clone(),
            sound_on: false,
            terminal_1: terminal_1_ref.clone(),
            terminal_2: terminal_2_ref.clone(),
            thread_handle: std::thread::spawn(move || {
                // _stream must live as long as the sink
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let terminal1_sinks = vec![
                    Sink::try_new(&stream_handle).unwrap(),
                    Sink::try_new(&stream_handle).unwrap(),
                    Sink::try_new(&stream_handle).unwrap(),
                    Sink::try_new(&stream_handle).unwrap(),
                ];

                let terminal2_sinks = vec![
                    Sink::try_new(&stream_handle).unwrap(),
                    Sink::try_new(&stream_handle).unwrap(),
                    Sink::try_new(&stream_handle).unwrap(),
                    Sink::try_new(&stream_handle).unwrap(),
                ];
                loop {
                    reproduce_sound_terminal(terminal_1_ref.clone(), &terminal1_sinks, "1");
                    reproduce_sound_terminal(terminal_2_ref.clone(), &terminal2_sinks, "2");

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
                terminal1.set_sounds(enabled_sounds_terminal_1, &mut self.sound_registers);
            }
            {
                let mut terminal2 = self.terminal_2.lock().unwrap();
                terminal2.set_volume(self.sound_registers.channel_control.terminal_2_volume());
                let enabled_sounds_terminal_2 = self
                    .sound_registers
                    .sound_output_selection
                    .terminal_2_sound();
                terminal2.set_sounds(enabled_sounds_terminal_2, &mut self.sound_registers);
            }
            self.sound_registers.load_in_ram(&mut ram);
        }
    }

    pub fn stop_sound_thread(&self) {
        let mut reference = self.thread_finished.lock().unwrap();
        *reference = true;
        std::thread::sleep(std::time::Duration::from_millis(32));
    }
}
