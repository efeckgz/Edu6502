use lib6502::bus::Bus;
use lib6502::cpu::Cpu;
use std::sync::Mutex;
use tauri::Manager;
mod api;

use api::commands;
use api::{Devices, Ram};

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
        .invoke_handler(tauri::generate_handler![commands::get_registers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
