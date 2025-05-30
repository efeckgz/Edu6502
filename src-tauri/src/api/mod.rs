use lib6502::bus::{Bus, BusDevice};
use lib6502::cpu::{self, Cpu};

use std::sync::Mutex;

pub mod commands;

static ROM: [u8; include_bytes!("a.out").len()] = *include_bytes!("a.out");

// The state of the application, managed by tauri::Manager
pub struct AppState {
    pub cpu: Cpu<Devices, 1>,
    pub registers: cpu::RegisterState,
    pub running: bool, // Flag to check if the emulator is running.
}

impl AppState {
    fn new(cpu: Cpu<Devices, 1>, registers: cpu::RegisterState) -> Self {
        AppState {
            cpu,
            registers,
            running: true, // Actually this needs to be false.
        }
    }
}

pub struct Ram {
    bytes: [u8; 65536],
}

impl Ram {
    pub fn new() -> Self {
        Self { bytes: [0; 65536] }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (addr, byte) in program.iter().enumerate() {
            self.write(addr as u16, *byte);
        }
    }
}

impl BusDevice for Ram {
    fn read(&mut self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bytes[addr as usize] = data;
    }
}

pub enum Devices {
    Ram(Ram),
}

impl BusDevice for Devices {
    fn read(&mut self, addr: u16) -> u8 {
        match self {
            Devices::Ram(ram) => ram.read(addr),
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match self {
            Devices::Ram(ram) => ram.write(addr, data),
        }
    }
}

pub fn initialize() -> Mutex<AppState> {
    let mut bus: Bus<Devices, 1> = Bus::new();

    let mut ram_inner = Ram::new();

    // Load hard coded program during initialization.
    // DO NOT DO THIS. THIS IS FOR TESTING ONLY. DO NOT LOAD HARD CODED PROGRAMS.
    ram_inner.load_program(&ROM);

    let ram = Devices::Ram(ram_inner);
    bus.map_device(0x0000, 0xFFFF, ram).unwrap();

    // Give initial register values by hand
    Mutex::new(AppState::new(Cpu::new(bus), (0, 255, 0, 0, 0, 0)))
}
