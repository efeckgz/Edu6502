use lib6502::bus::{Bus, BusDevice};
use lib6502::cpu::Cpu;

use std::sync::Mutex;

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

pub fn initialize() -> Mutex<Cpu<Devices, 1>> {
    let mut bus: Bus<Devices, 1> = Bus::new();
    let ram = Devices::Ram(Ram::new());
    bus.map_device(0x0000, 0xFFFF, ram).unwrap();

    Mutex::new(Cpu::new(bus))
}
