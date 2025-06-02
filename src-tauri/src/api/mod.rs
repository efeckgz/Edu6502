pub mod commands;

use lib6502::bus::Bus;
use lib6502::cpu::Cpu;
use std::sync::Mutex;
use tauri::{ipc::Channel, Manager, Result};

mod types;

use types::{AppState, Devices, InternalState, Ram};

// Initialize the state to manage using tauri::Manager
pub fn initialize() -> Mutex<AppState> {
    let mut bus: Bus<Devices, 1> = Bus::new();

    let ram = Devices::Ram(Ram::new());
    bus.map_device(0x0000, 0xFFFF, ram, 1).unwrap();

    // Give initial register values by hand
    Mutex::new(AppState::new(Cpu::new(bus)))
}

// Stream the cpu internal state using tauri::ipc::Channel to the frontend.
pub fn stream_cpu_state(cpu: &Cpu<Devices, 1>, chan: &Channel<InternalState>) -> Result<()> {
    let to_send = InternalState::new(cpu.get_state(), cpu.get_bus_pins());
    chan.send(to_send)
}

// Ensure that the assembler is installed.
pub fn check_install_assembler(app: &tauri::App) -> Result<()> {
    let mut dir = app.path().app_data_dir()?;
    dir.push("assembler");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    dir.push("vasm6502_oldstyle");
    if dir.is_file() {
        // println!("Yay!")
    }

    Ok(())
}
