// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::{Manager, State};

use lib6502::bus::{Bus, BusDevice};
use lib6502::cpu::Cpu;

use std::sync::Mutex;

struct Ram {
    bytes: [u8; 65536],
}

impl Ram {
    fn new() -> Self {
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

enum Devices {
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

#[tauri::command]
fn get_registers(state: State<'_, Mutex<Cpu<Devices, 1>>>) -> lib6502::cpu::RegisterState {
    let cpu = state.lock().unwrap();
    cpu.get_state()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let mut bus: Bus<Devices, 1> = Bus::new();
            let ram = Devices::Ram(Ram::new());
            bus.map_device(0x0000, 0xFFFF, ram).unwrap();

            app.manage(Mutex::new(Cpu::new(bus)));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_registers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
