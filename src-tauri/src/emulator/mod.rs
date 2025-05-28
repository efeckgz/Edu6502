use std::sync::Mutex;

use lib6502::bus::Bus;
use lib6502::cpu::Cpu;

mod ram;

use ram::Ram;

pub enum Devices {
    Ram(Ram),
}

impl lib6502::bus::BusDevice for Devices {
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

pub struct AppState {
    pub cpu: &'static mut Cpu<'static>,
}

pub fn initialize(bus: &mut Bus<Devices, 1>) -> Mutex<AppState> {
    let ram_inner = Ram::new();

    let ram_device = Devices::Ram(ram_inner);

    bus.map_device(0x0000, 0xFFFF, ram_device).unwrap();

    let mut cpu = Cpu::new(bus);
    Mutex::new(AppState { cpu: &mut cpu })
}
