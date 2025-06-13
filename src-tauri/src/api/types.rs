use lib6502::bus::BusDevice;
use lib6502::cpu::{self, Cpu};

const RAM_SIZE: usize = 65536;

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
    pub fn new(cpu: Cpu<Devices, 1>) -> Self {
        AppState {
            cpu,
            running: false,
        }
    }
}

pub struct Ram {
    bytes: [u8; RAM_SIZE],
}

impl Ram {
    pub fn new() -> Self {
        let mut bytes = [0; RAM_SIZE];

        // Set the reset vector to start exectuion from 0x0600
        bytes[0xFFFC] = 0x00;
        bytes[0xFFFD] = 0x06;

        Self { bytes }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (addr, byte) in program.iter().enumerate() {
            self.write(addr as u16 + 0x0600, *byte);
        }
    }

    // Reset the ram to the 0 state.
    pub fn reset(&mut self) {
        self.bytes = {
            let mut _bytes = [0; RAM_SIZE];

            // Set the reset vector to start exectuion from 0x0600
            _bytes[0xFFFC] = 0x00;
            _bytes[0xFFFD] = 0x06;
            _bytes
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
