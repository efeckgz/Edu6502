const MEMSIZE: usize = 65536;

use lib6502::bus::BusDevice;

pub struct Ram {
    bytes: [u8; MEMSIZE],
}

impl BusDevice for Ram {
    fn read(&mut self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bytes[addr as usize] = data;
    }
}

impl Ram {
    pub fn new() -> Self {
        Self {
            bytes: [0_u8; MEMSIZE],
        }
    }

    pub fn load_program(&mut self, from: usize, program: &[u8]) {
        for (addr, byte) in program.iter().enumerate() {
            if addr + from > MEMSIZE {
                return;
            }
            self.bytes[from + addr] = *byte;
        }
    }
}
