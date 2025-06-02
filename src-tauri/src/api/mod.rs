use lib6502::bus::{Bus, BusDevice};
use lib6502::cpu::{self, Cpu};
use tauri::{ipc::Channel, Result};

use std::sync::Mutex;

pub mod commands;

pub static ROM: [u8; include_bytes!("a.out").len()] = *include_bytes!("a.out");

// The state of the application, managed by tauri::Manager
pub struct AppState {
    pub cpu: Cpu<Devices, 1>,
    pub running: bool, // Flag to check if the emulator is running.
}

// Internal cpu state, constists of cpu registers and bus pins.
// Sent via tauri::ipc::Channel while the cpu is running.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InternalState {
    // Registers
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,

    // Bus pins
    addr: u16,
    data: u8,
    rw: bool,
}

impl InternalState {
    pub fn new(regs: cpu::RegisterState, bus: (u16, u8, bool)) -> InternalState {
        let (pc, s, a, x, y, p) = regs;
        let (addr, data, rw) = bus;
        InternalState {
            pc,
            s,
            a,
            x,
            y,
            p,
            addr,
            data,
            rw,
        }
    }
}

impl AppState {
    fn new(cpu: Cpu<Devices, 1>) -> Self {
        AppState {
            cpu,
            running: false,
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

    // Reset the ram to the 0 state.
    pub fn reset(&mut self) {
        self.bytes = [0; 65536]
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

// Initialize the state to manage using tauri::Manager
pub fn initialize() -> Mutex<AppState> {
    let mut bus: Bus<Devices, 1> = Bus::new();

    let mut ram_inner = Ram::new();

    // Load hard coded program during initialization.
    // DO NOT DO THIS. THIS IS FOR TESTING ONLY. DO NOT LOAD HARD CODED PROGRAMS.
    ram_inner.load_program(&ROM);

    let ram = Devices::Ram(ram_inner);
    bus.map_device(0x0000, 0xFFFF, ram, 1).unwrap();

    // Give initial register values by hand
    Mutex::new(AppState::new(Cpu::new(bus)))
}

// Stream the cpu internal state using tauri::ipc::Channel to the frontend.
pub fn stream_cpu_state(cpu: &Cpu<Devices, 1>, chan: &Channel<InternalState>) -> Result<()> {
    let to_send = InternalState::new(cpu.get_state(), cpu.get_bus_pins());
    chan.send(to_send)
}
