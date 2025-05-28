use lib6502::bus::{Bus, BusDevice};
use lib6502::cpu::Cpu;

pub mod commands;

pub struct Ram {
    bytes: [u8; 65536],
}

impl Ram {
    pub fn new() -> Self {
        Self { bytes: [0; 65536] }
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
