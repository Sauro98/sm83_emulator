pub mod clock;
pub mod lcd_controller;
pub mod ram;
pub mod sm83;

use ram::MemoryRegister;
use sm83::snapshot::SM83Snapshot;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

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
    bootlock_register: ram::BootLockMemoryRegister,
    lcd_controller: Arc<Mutex<lcd_controller::LCDController>>,
}

impl System {
    pub fn new(clock_frequency: f32) -> System {
        return System {
            cpu: Arc::new(Mutex::new(sm83::SM83::new(clock_frequency))),
            ram: Arc::new(Mutex::new(ram::RAM::new())),
            boot_rom: ram::BootRom::new(),
            bootlock_register: ram::BootLockMemoryRegister::new(),
            lcd_controller: Arc::new(Mutex::new(lcd_controller::LCDController::new())),
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
        return System {
            cpu: Arc::new(Mutex::new(cpu)),
            ram: Arc::new(Mutex::new(ram)),
            boot_rom: ram::BootRom::new(),
            bootlock_register: ram::BootLockMemoryRegister::new(),
            lcd_controller: Arc::new(Mutex::new(lcd_controller::LCDController::new())),
        };
    }

    pub fn next(&mut self) {
        let mut ram = self.ram.lock().unwrap();
        let mut cpu = self.cpu.lock().unwrap();
        cpu.next(&mut ram);
        if ram.was_dma_requested() {
            //todo perform dma
            println!("Performing DMA");
            ram.reset_dma_request();
        }
    }

    pub fn run(mut self, n_iter: usize) -> Self {
        self.boot();
        let cpu_ram_ref = self.ram.clone();
        let cpu_ref = self.cpu.clone();
        let lcd_ram_ref = self.ram.clone();
        let lcd_ref = self.lcd_controller.clone();
        let loop_finished_ref = Arc::new(AtomicBool::new(false));
        let lcd_loop_finished_ref = loop_finished_ref.clone();
        let cpu_thread_handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            for _ in 0..n_iter {
                let mut ram = cpu_ram_ref.lock().unwrap();
                let mut cpu = cpu_ref.lock().unwrap();
                cpu.next(&mut ram);
                if ram.was_dma_requested() {
                    //todo perform dma
                    println!("Performing DMA");
                    ram.reset_dma_request();
                }
                if cpu.get_register(sm83::registers::RegisterName::PC) >= 0xFF {
                    println!("boot rom ended");
                }
            }
            loop_finished_ref.store(true, Ordering::Relaxed);
            let cpu = cpu_ref.lock().unwrap();
            println!(
                "CPU pc: {:X}",
                cpu.get_register(sm83::registers::RegisterName::PC)
            );
            let dur = std::time::Instant::now().duration_since(start);
            let elapsed_nanos = dur.as_nanos() as f64;
            let cycles_per_nano = (n_iter as f64) / elapsed_nanos;
            let cycles_per_second = cycles_per_nano * 1e9;
            println!(
                "CPU Execution frequency {}",
                format_frequency(cycles_per_second as f32)
            );
            println!("{}", cpu.sleep_count());
        });
        let lcd_thread_handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            loop {
                let mut lcd = lcd_ref.lock().unwrap();
                lcd.next(&lcd_ram_ref);
                if lcd_loop_finished_ref.load(Ordering::Relaxed) {
                    break;
                }
            }
            let dur = std::time::Instant::now().duration_since(start);
            let elapsed_nanos = dur.as_nanos() as f64;
            let cycles_per_nano = (n_iter as f64) / elapsed_nanos;
            let cycles_per_second = cycles_per_nano * 1e9;
            println!(
                "LCD Execution frequency {}",
                format_frequency(cycles_per_second as f32)
            );
            lcd_ref.lock().unwrap().stop_window_thread();
        });
        println!("Wating for join");
        cpu_thread_handle.join().unwrap();
        println!("CPU joined");
        lcd_thread_handle.join().unwrap();
        println!("LCD joined");

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
        self.bootlock_register.lock();
        self.boot_rom.load_in_ram(&mut ram);
        self.bootlock_register.load_in_ram(&mut ram);
        let fakerom = ram::FakeRom::new();
        fakerom.load_in_ram(&mut ram);
        cpu.reset(&ram);
    }
}
