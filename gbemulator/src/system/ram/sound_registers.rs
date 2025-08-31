use super::{
    default_memory_register_trait_impl, default_nonimplemented_memory_register_trait_impl,
    MemoryRegister,
};

const CHANNEL_1_SWEEP_REGISTER_ADDRESS: u16 = 0xFF10;
const CHANNEL_1_WAVE_PATTERN_ADDRESS: u16 = 0xFF11;
const CHANNEL_1_VOLUME_ENVELOPE_ADDRESS: u16 = 0xFF12;
const CHANNEL_1_FREQUENCY_LO_ADDRESS: u16 = 0xFF13;
const CHANNEL_1_FREQUENCY_HI_ADDRESS: u16 = 0xFF14;

const CHANNEL_2_WAVE_PATTERN_ADDRESS: u16 = 0xFF16;
const CHANNEL_2_VOLUME_ENVELOPE_ADDRESS: u16 = 0xFF17;
const CHANNEL_2_FREQUENCY_LO_ADDRESS: u16 = 0xFF18;
const CHANNEL_2_FREQUENCY_HI_ADDRESS: u16 = 0xFF19;

const CHANNEL_3_SOUND_ON_ADDRESS: u16 = 0xFF1A;
const CHANNEL_3_SOUND_LENGTH_ADDRESS: u16 = 0xFF1B;
const CHANNEL_3_OUTPUT_LEVEL_ADDRESS: u16 = 0xFF1C;
const CHANNEL_3_FREQUENCY_LO_ADDRESS: u16 = 0xFF1D;
const CHANNEL_3_FREQUENCY_HI_ADDRESS: u16 = 0xFF1E;

const CHANNEL_4_SOUND_LENGTH_ADDRESS: u16 = 0xFF20;
const CHANNEL_4_VOLUME_ENVELOPE_ADDRESS: u16 = 0xFF21;
const CHANNEL_4_POLYNOMIAL_COUNTER_ADDRESS: u16 = 0xFF22;
const CHANNEL_4_COUNTER_CONSECUTIVE_ADDRESS: u16 = 0xFF23;

const CHANNEL_CONTROL_ADDRESS: u16 = 0xFF24;
const SOUND_OUTPUT_SELECTION_ADDRESS: u16 = 0xFF25;
const SOUND_ON_OFF_ADDRESS: u16 = 0xFF26;

pub struct Channel1SweepRegister {
    value: u8,
    address: u16,
}

impl Channel1SweepRegister {
    pub fn new() -> Self {
        Channel1SweepRegister {
            address: CHANNEL_1_SWEEP_REGISTER_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel1WavePatternRegister {
    value: u8,
    address: u16,
}

impl Channel1WavePatternRegister {
    pub fn new() -> Self {
        Channel1WavePatternRegister {
            address: CHANNEL_1_WAVE_PATTERN_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel1VolumeEnvelopeRegister {
    value: u8,
    address: u16,
}

impl Channel1VolumeEnvelopeRegister {
    pub fn new() -> Self {
        Channel1VolumeEnvelopeRegister {
            address: CHANNEL_1_VOLUME_ENVELOPE_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel1FrequencyLoRegister {
    value: u8,
    address: u16,
}

impl Channel1FrequencyLoRegister {
    pub fn new() -> Self {
        Channel1FrequencyLoRegister {
            address: CHANNEL_1_FREQUENCY_LO_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel1FrequencyHiRegister {
    value: u8,
    address: u16,
}

impl Channel1FrequencyHiRegister {
    pub fn new() -> Self {
        Channel1FrequencyHiRegister {
            address: CHANNEL_1_FREQUENCY_HI_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel2WavePatternRegister {
    value: u8,
    address: u16,
}

impl Channel2WavePatternRegister {
    pub fn new() -> Self {
        Channel2WavePatternRegister {
            address: CHANNEL_2_WAVE_PATTERN_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel2VolumeEnvelopeRegister {
    value: u8,
    address: u16,
}

impl Channel2VolumeEnvelopeRegister {
    pub fn new() -> Self {
        Channel2VolumeEnvelopeRegister {
            address: CHANNEL_2_VOLUME_ENVELOPE_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel2FrequencyLoRegister {
    value: u8,
    address: u16,
}

impl Channel2FrequencyLoRegister {
    pub fn new() -> Self {
        Channel2FrequencyLoRegister {
            address: CHANNEL_2_FREQUENCY_LO_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel2FrequencyHiRegister {
    value: u8,
    address: u16,
}

impl Channel2FrequencyHiRegister {
    pub fn new() -> Self {
        Channel2FrequencyHiRegister {
            address: CHANNEL_2_FREQUENCY_HI_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel3SoundOnRegister {
    value: u8,
    address: u16,
}

impl Channel3SoundOnRegister {
    pub fn new() -> Self {
        Channel3SoundOnRegister {
            address: CHANNEL_3_SOUND_ON_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel3SoundLengthRegister {
    value: u8,
    address: u16,
}

impl Channel3SoundLengthRegister {
    pub fn new() -> Self {
        Channel3SoundLengthRegister {
            address: CHANNEL_3_SOUND_LENGTH_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel3OutputLevelRegister {
    value: u8,
    address: u16,
}

impl Channel3OutputLevelRegister {
    pub fn new() -> Self {
        Channel3OutputLevelRegister {
            address: CHANNEL_3_OUTPUT_LEVEL_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel3FrequencyLoRegister {
    value: u8,
    address: u16,
}

impl Channel3FrequencyLoRegister {
    pub fn new() -> Self {
        Channel3FrequencyLoRegister {
            address: CHANNEL_3_FREQUENCY_LO_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel3FrequencyHiRegister {
    value: u8,
    address: u16,
}

impl Channel3FrequencyHiRegister {
    pub fn new() -> Self {
        Channel3FrequencyHiRegister {
            address: CHANNEL_3_FREQUENCY_HI_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel4SoundLengthRegister {
    value: u8,
    address: u16,
}

impl Channel4SoundLengthRegister {
    pub fn new() -> Self {
        Channel4SoundLengthRegister {
            address: CHANNEL_4_SOUND_LENGTH_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel4VolumeEnvelopeRegister {
    value: u8,
    address: u16,
}

impl Channel4VolumeEnvelopeRegister {
    pub fn new() -> Self {
        Channel4VolumeEnvelopeRegister {
            address: CHANNEL_4_VOLUME_ENVELOPE_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel4PolynomialCounterRegister {
    value: u8,
    address: u16,
}

impl Channel4PolynomialCounterRegister {
    pub fn new() -> Self {
        Channel4PolynomialCounterRegister {
            address: CHANNEL_4_POLYNOMIAL_COUNTER_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct Channel4CounterConsecutiveRegister {
    value: u8,
    address: u16,
}

impl Channel4CounterConsecutiveRegister {
    pub fn new() -> Self {
        Channel4CounterConsecutiveRegister {
            address: CHANNEL_4_COUNTER_CONSECUTIVE_ADDRESS,
            value: 0x0,
        }
    }
}

pub struct ChannelControlRegister {
    value: u8,
    address: u16,
}

impl ChannelControlRegister {
    pub fn new() -> Self {
        ChannelControlRegister {
            address: CHANNEL_CONTROL_ADDRESS,
            value: 0x0,
        }
    }

    pub fn terminal_1_volume(&self) -> u8 {
        self.value & 0x03
    }

    pub fn terminal_2_volume(&self) -> u8 {
        (self.value & 0x30) >> 4
    }
}

pub struct SoundOutputSelectionRegister {
    value: u8,
    address: u16,
}

impl SoundOutputSelectionRegister {
    pub fn new() -> Self {
        SoundOutputSelectionRegister {
            address: SOUND_OUTPUT_SELECTION_ADDRESS,
            value: 0x0,
        }
    }

    pub fn terminal_1_sound(&self) -> [bool; 4] {
        [
            self.value & 0x01 != 0,
            self.value & 0x02 != 0,
            self.value & 0x04 != 0,
            self.value & 0x08 != 0,
        ]
    }

    pub fn terminal_2_sound(&self) -> [bool; 4] {
        [
            self.value & 0x10 != 0,
            self.value & 0x20 != 0,
            self.value & 0x40 != 0,
            self.value & 0x80 != 0,
        ]
    }
}

pub struct SoundOnOffRegister {
    value: u8,
    address: u16,
}

impl SoundOnOffRegister {
    pub fn new() -> Self {
        SoundOnOffRegister {
            address: SOUND_ON_OFF_ADDRESS,
            value: 0x0,
        }
    }

    pub fn is_sound_on(&self) -> bool {
        self.value & 0x80 != 0
    }
}

default_memory_register_trait_impl!(Channel1SweepRegister, 0x0);
default_memory_register_trait_impl!(Channel1WavePatternRegister, 0x0);
default_memory_register_trait_impl!(Channel1VolumeEnvelopeRegister, 0x0);
default_memory_register_trait_impl!(Channel1FrequencyLoRegister, 0x0);
default_memory_register_trait_impl!(Channel1FrequencyHiRegister, 0x0);
default_memory_register_trait_impl!(Channel2WavePatternRegister, 0x0);
default_memory_register_trait_impl!(Channel2VolumeEnvelopeRegister, 0x0);
default_memory_register_trait_impl!(Channel2FrequencyLoRegister, 0x0);
default_memory_register_trait_impl!(Channel2FrequencyHiRegister, 0x0);
default_memory_register_trait_impl!(Channel3SoundOnRegister, 0x0);
default_memory_register_trait_impl!(Channel3SoundLengthRegister, 0x0);
default_memory_register_trait_impl!(Channel3OutputLevelRegister, 0x0);
default_memory_register_trait_impl!(Channel3FrequencyLoRegister, 0x0);
default_memory_register_trait_impl!(Channel3FrequencyHiRegister, 0x0);
default_memory_register_trait_impl!(Channel4SoundLengthRegister, 0x0);
default_memory_register_trait_impl!(Channel4VolumeEnvelopeRegister, 0x0);
default_memory_register_trait_impl!(Channel4PolynomialCounterRegister, 0x0);
default_memory_register_trait_impl!(Channel4CounterConsecutiveRegister, 0x0);
default_memory_register_trait_impl!(ChannelControlRegister, 0x0);
default_memory_register_trait_impl!(SoundOutputSelectionRegister, 0x0);
default_memory_register_trait_impl!(SoundOnOffRegister, 0x0);

pub struct Channel1Registers {
    pub sweep: Channel1SweepRegister,
    pub wave_pattern: Channel1WavePatternRegister,
    pub volume_envelope: Channel1VolumeEnvelopeRegister,
    pub frequency_lo: Channel1FrequencyLoRegister,
    pub frequency_hi: Channel1FrequencyHiRegister,
}

impl Channel1Registers {
    pub fn new() -> Self {
        Channel1Registers {
            sweep: Channel1SweepRegister::new(),
            wave_pattern: Channel1WavePatternRegister::new(),
            volume_envelope: Channel1VolumeEnvelopeRegister::new(),
            frequency_lo: Channel1FrequencyLoRegister::new(),
            frequency_hi: Channel1FrequencyHiRegister::new(),
        }
    }
}

impl MemoryRegister for Channel1Registers {
    fn read_from_ram(&mut self, ram: &super::RAM) {
        self.sweep.read_from_ram(ram);
        self.wave_pattern.read_from_ram(ram);
        self.volume_envelope.read_from_ram(ram);
        self.frequency_lo.read_from_ram(ram);
        self.frequency_hi.read_from_ram(ram);
    }

    fn load_in_ram(&self, ram: &mut super::RAM) -> Option<()> {
        self.sweep.load_in_ram(ram).unwrap();
        self.wave_pattern.load_in_ram(ram).unwrap();
        self.volume_envelope.load_in_ram(ram).unwrap();
        self.frequency_lo.load_in_ram(ram).unwrap();
        self.frequency_hi.load_in_ram(ram)
    }

    fn reset(&mut self) {
        self.sweep.reset();
        self.wave_pattern.reset();
        self.volume_envelope.reset();
        self.frequency_lo.reset();
        self.frequency_hi.reset();
    }
    default_nonimplemented_memory_register_trait_impl!();
}

pub struct Channel2Registers {
    pub wave_pattern: Channel2WavePatternRegister,
    pub volume_envelope: Channel2VolumeEnvelopeRegister,
    pub frequency_lo: Channel2FrequencyLoRegister,
    pub frequency_hi: Channel2FrequencyHiRegister,
}

impl Channel2Registers {
    pub fn new() -> Self {
        Channel2Registers {
            wave_pattern: Channel2WavePatternRegister::new(),
            volume_envelope: Channel2VolumeEnvelopeRegister::new(),
            frequency_lo: Channel2FrequencyLoRegister::new(),
            frequency_hi: Channel2FrequencyHiRegister::new(),
        }
    }
}

impl MemoryRegister for Channel2Registers {
    default_nonimplemented_memory_register_trait_impl!();

    fn read_from_ram(&mut self, ram: &super::RAM) {
        self.wave_pattern.read_from_ram(ram);
        self.volume_envelope.read_from_ram(ram);
        self.frequency_lo.read_from_ram(ram);
        self.frequency_hi.read_from_ram(ram);
    }

    fn load_in_ram(&self, ram: &mut super::RAM) -> Option<()> {
        self.wave_pattern.load_in_ram(ram).unwrap();
        self.volume_envelope.load_in_ram(ram).unwrap();
        self.frequency_lo.load_in_ram(ram).unwrap();
        self.frequency_hi.load_in_ram(ram)
    }

    fn reset(&mut self) {
        self.wave_pattern.reset();
        self.volume_envelope.reset();
        self.frequency_lo.reset();
        self.frequency_hi.reset();
    }
}

pub struct Channel3Registers {
    pub sound_on: Channel3SoundOnRegister,
    pub sound_length: Channel3SoundLengthRegister,
    pub output_level: Channel3OutputLevelRegister,
    pub frequency_lo: Channel3FrequencyLoRegister,
    pub frequency_hi: Channel3FrequencyHiRegister,
}

impl Channel3Registers {
    pub fn new() -> Self {
        Channel3Registers {
            sound_on: Channel3SoundOnRegister::new(),
            sound_length: Channel3SoundLengthRegister::new(),
            output_level: Channel3OutputLevelRegister::new(),
            frequency_lo: Channel3FrequencyLoRegister::new(),
            frequency_hi: Channel3FrequencyHiRegister::new(),
        }
    }
}

impl MemoryRegister for Channel3Registers {
    default_nonimplemented_memory_register_trait_impl!();

    fn read_from_ram(&mut self, ram: &super::RAM) {
        self.sound_on.read_from_ram(ram);
        self.sound_length.read_from_ram(ram);
        self.output_level.read_from_ram(ram);
        self.frequency_lo.read_from_ram(ram);
        self.frequency_hi.read_from_ram(ram);
    }

    fn load_in_ram(&self, ram: &mut super::RAM) -> Option<()> {
        self.sound_on.load_in_ram(ram).unwrap();
        self.sound_length.load_in_ram(ram).unwrap();
        self.output_level.load_in_ram(ram).unwrap();
        self.frequency_lo.load_in_ram(ram).unwrap();
        self.frequency_hi.load_in_ram(ram)
    }

    fn reset(&mut self) {
        self.sound_on.reset();
        self.sound_length.reset();
        self.output_level.reset();
        self.frequency_lo.reset();
        self.frequency_hi.reset();
    }
}

pub struct Channel4Registers {
    pub sound_length: Channel4SoundLengthRegister,
    pub volume_envelope: Channel4VolumeEnvelopeRegister,
    pub polynomial_counter: Channel4PolynomialCounterRegister,
    pub counter_consecutive: Channel4CounterConsecutiveRegister,
}

impl Channel4Registers {
    pub fn new() -> Self {
        Channel4Registers {
            sound_length: Channel4SoundLengthRegister::new(),
            volume_envelope: Channel4VolumeEnvelopeRegister::new(),
            polynomial_counter: Channel4PolynomialCounterRegister::new(),
            counter_consecutive: Channel4CounterConsecutiveRegister::new(),
        }
    }
}

impl MemoryRegister for Channel4Registers {
    default_nonimplemented_memory_register_trait_impl!();

    fn read_from_ram(&mut self, ram: &super::RAM) {
        self.sound_length.read_from_ram(ram);
        self.volume_envelope.read_from_ram(ram);
        self.polynomial_counter.read_from_ram(ram);
        self.counter_consecutive.read_from_ram(ram);
    }

    fn load_in_ram(&self, ram: &mut super::RAM) -> Option<()> {
        self.sound_length.load_in_ram(ram).unwrap();
        self.volume_envelope.load_in_ram(ram).unwrap();
        self.polynomial_counter.load_in_ram(ram).unwrap();
        self.counter_consecutive.load_in_ram(ram)
    }

    fn reset(&mut self) {
        self.sound_length.reset();
        self.volume_envelope.reset();
        self.polynomial_counter.reset();
        self.counter_consecutive.reset();
    }
}

pub struct SoundRegisters {
    pub channel_1: Channel1Registers,
    pub channel_2: Channel2Registers,
    pub channel_3: Channel3Registers,
    pub channel_4: Channel4Registers,
    pub channel_control: ChannelControlRegister,
    pub sound_output_selection: SoundOutputSelectionRegister,
    pub sound_on_off: SoundOnOffRegister,
}

impl SoundRegisters {
    pub fn new() -> Self {
        SoundRegisters {
            channel_1: Channel1Registers::new(),
            channel_2: Channel2Registers::new(),
            channel_3: Channel3Registers::new(),
            channel_4: Channel4Registers::new(),
            channel_control: ChannelControlRegister::new(),
            sound_output_selection: SoundOutputSelectionRegister::new(),
            sound_on_off: SoundOnOffRegister::new(),
        }
    }
}

impl MemoryRegister for SoundRegisters {
    default_nonimplemented_memory_register_trait_impl!();

    fn read_from_ram(&mut self, ram: &super::RAM) {
        self.channel_1.read_from_ram(ram);
        self.channel_2.read_from_ram(ram);
        self.channel_3.read_from_ram(ram);
        self.channel_4.read_from_ram(ram);
        self.channel_control.read_from_ram(ram);
        self.sound_output_selection.read_from_ram(ram);
        self.sound_on_off.read_from_ram(ram);
    }

    fn load_in_ram(&self, ram: &mut super::RAM) -> Option<()> {
        self.channel_1.load_in_ram(ram).unwrap();
        self.channel_2.load_in_ram(ram).unwrap();
        self.channel_3.load_in_ram(ram).unwrap();
        self.channel_4.load_in_ram(ram).unwrap();
        self.channel_control.load_in_ram(ram).unwrap();
        self.sound_output_selection.load_in_ram(ram).unwrap();
        self.sound_on_off.load_in_ram(ram)
    }

    fn reset(&mut self) {
        self.channel_1.reset();
        self.channel_2.reset();
        self.channel_3.reset();
        self.channel_4.reset();
        self.channel_control.reset();
        self.sound_output_selection.reset();
        self.sound_on_off.reset();
    }
}
