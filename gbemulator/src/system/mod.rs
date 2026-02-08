pub mod controllers;
pub mod master_clock;
pub mod ram;
pub mod sm83;

use master_clock::MasterClock;
use ram::MemoryRegister;
use sm83::snapshot::SM83Snapshot;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

use crate::system::ram::mapping_chip::DynamicMappingChip;

fn format_frequency(frequency: f32) -> String {
    if frequency < 1e3 {
        return format!("{:.2} Hz", frequency);
    } else if frequency < 1e6 {
        return format!("{:.2} kHz", frequency / 1e3);
    } else if frequency < 1e9 {
        return format!("{:.2} MHz", frequency / 1e6);
    } else if frequency < 1e12 {
        return format!("{:.2} GHz", frequency / 1e9);
    } else {
        return "Impossible frequency".to_string();
    }
}

pub struct System {
    cpu: Arc<Mutex<sm83::SM83>>,
    ram: Arc<Mutex<ram::RAM>>,
    boot_rom: ram::BootRom,
    bootlock_register: Arc<Mutex<ram::BootLockMemoryRegister>>,
    lcd_controller: Arc<Mutex<controllers::lcd_controller::LCDController>>,
    sound_controller: Arc<Mutex<controllers::sound_controller::SoundController>>,
    should_reload_cartridge: bool,
    master_clock: MasterClock,
    cpu_ready: Arc<AtomicBool>,
    lcd_ready: Arc<AtomicBool>,
    sound_ready: Arc<AtomicBool>,
}

impl System {
    pub fn new(dynamic_chip: Option<DynamicMappingChip>, headless: bool) -> System {
        return System {
            cpu: Arc::new(Mutex::new(sm83::SM83::new())),
            ram: Arc::new(Mutex::new(ram::RAM::new(dynamic_chip))),
            boot_rom: ram::BootRom::new(),
            bootlock_register: Arc::new(Mutex::new(ram::BootLockMemoryRegister::new())),
            lcd_controller: Arc::new(Mutex::new(controllers::lcd_controller::LCDController::new(
                headless,
            ))),
            sound_controller: Arc::new(Mutex::new(
                controllers::sound_controller::SoundController::new(),
            )),
            should_reload_cartridge: false,
            master_clock: MasterClock::new(),
            cpu_ready: Arc::new(AtomicBool::new(false)),
            lcd_ready: Arc::new(AtomicBool::new(false)),
            sound_ready: Arc::new(AtomicBool::new(false)),
        };
    }

    pub fn from_ram_snapshot(ram: ram::RAM, snapshot: SM83Snapshot, headless: bool) -> System {
        let mut cpu = sm83::SM83::new();
        cpu.load_snapshot(snapshot);
        cpu.fetch_cycle(&ram);
        return System {
            cpu: Arc::new(Mutex::new(cpu)),
            ram: Arc::new(Mutex::new(ram)),
            boot_rom: ram::BootRom::new(),
            bootlock_register: Arc::new(Mutex::new(ram::BootLockMemoryRegister::new())),
            lcd_controller: Arc::new(Mutex::new(controllers::lcd_controller::LCDController::new(
                headless,
            ))),
            sound_controller: Arc::new(Mutex::new(
                controllers::sound_controller::SoundController::new(),
            )),
            should_reload_cartridge: false,
            master_clock: MasterClock::new(),
            cpu_ready: Arc::new(AtomicBool::new(false)),
            lcd_ready: Arc::new(AtomicBool::new(false)),
            sound_ready: Arc::new(AtomicBool::new(false)),
        };
    }

    pub fn next(&mut self) {
        let mut ram = self.ram.lock().unwrap();
        let mut cpu = self.cpu.lock().unwrap();
        cpu.next(&mut ram);
    }

    pub fn run(mut self, n_iter: usize) -> Self {
        self.boot();
        let cpu_ram_ref: Arc<Mutex<ram::RAM>> = self.ram.clone();
        let bootlocker_ref = self.bootlock_register.clone();
        let cpu_ref = self.cpu.clone();
        let lcd_ram_ref = self.ram.clone();
        let lcd_ref = self.lcd_controller.clone();
        let sound_ram_ref = self.ram.clone();
        let sound_ref = self.sound_controller.clone();
        let loop_finished_ref = Arc::new(AtomicBool::new(false));
        let cpu_loop_finished_ref = loop_finished_ref.clone();
        let lcd_loop_finished_ref = loop_finished_ref.clone();
        let sound_loop_finished_ref = loop_finished_ref.clone();
        let cpu_ready_ref = self.cpu_ready.clone();
        let lcd_ready_ref = self.lcd_ready.clone();
        let sound_ready_ref = self.sound_ready.clone();
        let cpu_thread_handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            for _ in 0..n_iter {
                if cpu_ready_ref.load(Ordering::Relaxed) {
                    let mut ram = cpu_ram_ref.lock().unwrap();
                    let mut cpu = cpu_ref.lock().unwrap();
                    cpu.next(&mut ram);

                    let mut blr = bootlocker_ref.lock().unwrap();
                    blr.read_from_ram(&ram);
                    if blr.is_unlocked() && self.should_reload_cartridge {
                        println!("bootlock unlocked");
                        self.should_reload_cartridge = false;
                        ram.load_base_rom_bank();
                    }

                    if cpu.get_register(sm83::registers::RegisterName::PC) == 0xFF {
                        println!("boot rom ended");
                    }
                }
            }
            cpu_loop_finished_ref.store(true, Ordering::Relaxed);
            let cpu = cpu_ref.lock().unwrap();
            println!(
                "CPU pc: {:X}",
                cpu.get_register(sm83::registers::RegisterName::PC)
            );
            let dur = std::time::Instant::now().duration_since(start);
            let elapsed_nanos = dur.as_nanos() as f64;
            let cycles_per_nano = (cpu.cycle_count as f64) / elapsed_nanos;
            let cycles_per_second = cycles_per_nano * 1e9;
            println!(
                "CPU Execution frequency {}",
                format_frequency(cycles_per_second as f32)
            );
        });
        let lcd_thread_handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut lcd_n_iter = 0;
            loop {
                if lcd_ready_ref.load(Ordering::Relaxed) {
                    let mut lcd = lcd_ref.lock().unwrap();
                    lcd.next(&lcd_ram_ref);
                    lcd_n_iter += 1;
                }
                if lcd_loop_finished_ref.load(Ordering::Relaxed) {
                    break;
                }
            }
            let dur = std::time::Instant::now().duration_since(start);
            let elapsed_nanos = dur.as_nanos() as f64;
            let cycles_per_nano = (lcd_n_iter as f64) / elapsed_nanos;
            let cycles_per_second = cycles_per_nano * 1e9;
            println!(
                "LCD Execution frequency {}",
                format_frequency(cycles_per_second as f32)
            );
            lcd_ref.lock().unwrap().stop_window_thread();
        });
        let sound_thread_handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            let mut sound_n_iter = 0;
            loop {
                if sound_ready_ref.load(Ordering::Relaxed) {
                    let mut sound = sound_ref.lock().unwrap();
                    sound.next(&sound_ram_ref);
                    sound_n_iter += 1;
                }
                if sound_loop_finished_ref.load(Ordering::Relaxed) {
                    break;
                }
            }
            let dur = std::time::Instant::now().duration_since(start);
            let elapsed_nanos = dur.as_nanos() as f64;
            let cycles_per_nano = (sound_n_iter as f64) / elapsed_nanos;
            let cycles_per_second = cycles_per_nano * 1e9;
            println!(
                "Sound Execution frequency {}",
                format_frequency(cycles_per_second as f32)
            );
            sound_ref.lock().unwrap().stop_sound_thread();
        });

        loop {
            self.master_clock
                .next(&self.cpu_ready, &self.lcd_ready, &self.sound_ready);
            if loop_finished_ref.load(Ordering::Relaxed) {
                break;
            }
        }

        println!("Wating for join");
        cpu_thread_handle.join().unwrap();
        println!("CPU joined");
        lcd_thread_handle.join().unwrap();
        println!("LCD joined");
        sound_thread_handle.join().unwrap();
        println!("Sound joined");

        self
    }

    pub fn cycle_count(&self) -> u128 {
        return self.cpu.lock().unwrap().cycle_count;
    }

    pub fn to_snapshot(&self) -> SM83Snapshot {
        return self.cpu.lock().unwrap().to_snapshot();
    }

    pub fn get_ram(&self) -> ram::RAM {
        self.ram.lock().unwrap().clone()
    }

    pub fn boot(&mut self) {
        let mut ram = self.ram.lock().unwrap();
        let mut cpu = self.cpu.lock().unwrap();
        ram.load_base_rom_bank();
        // override memory from 0000 to 00FF with boot rom
        self.boot_rom.load_in_ram(&mut ram);
        {
            let mut bootlock_register_ref = self.bootlock_register.lock().unwrap();
            bootlock_register_ref.lock();
            bootlock_register_ref.load_in_ram(&mut ram);
            self.should_reload_cartridge = true;
        }
        cpu.reset(&ram);
        self.master_clock.start();
    }
}
