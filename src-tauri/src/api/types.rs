use lib6502::bus::BusDevice;
use lib6502::cpu::{self, Cpu};

const RAM_SIZE: usize = 65536;

const fn init_base_ram() -> [u8; RAM_SIZE] {
    let mut bytes = [0xEA; RAM_SIZE];
    let mut i = 0;

    // Fill 0x0000 to 0x05FF with 0x00
    while i < 0x0600 {
        bytes[i] = 0x00;
        i += 1;
    }

    // Set reset vector
    bytes[0xFFFC] = 0x00;
    bytes[0xFFFD] = 0x06;

    bytes
}

const BASE_RAM: [u8; RAM_SIZE] = init_base_ram();

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
        Self { bytes: BASE_RAM }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        for (addr, byte) in program.iter().enumerate() {
            self.write(addr as u16 + 0x0600, *byte);
        }
    }

    // Reset the ram to the 0 state.
    pub fn reset(&mut self) {
        self.bytes = BASE_RAM;
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
