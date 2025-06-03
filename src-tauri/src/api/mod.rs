pub mod commands;

use lib6502::bus::Bus;
use lib6502::cpu::Cpu;
use reqwest;
use std::{fs::File, io::Write, os::unix::fs::PermissionsExt, path::PathBuf, sync::Mutex};
use tauri::{ipc::Channel, Manager, Result};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

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

    let bin_path = dir.clone().join("vasm6502_oldstyle");
    if bin_path.is_file() {
        return Ok(());
    }

    // Show dialog
    let _ = app
        .dialog()
        .message(
            "Downloading assembler. This is a one time operation. Do not close the application.",
        )
        .kind(MessageDialogKind::Info)
        .title("Downloading assembler")
        .show(|op| match op {
            true => return,
            false => return,
        });

    // Dr. Volker Barthelmann's vasm assembler.
    let url = "http://sun.hasenbraten.de/vasm/daily/vasm.tar.gz";
    let compressed = dir.join("vasm.tar.gz");
    download_file(url, &compressed)?;

    // Set executable permissions on Unix systems.
    #[cfg(unix)]
    {
        // let mut perms = std::fs::metadata(&compressed)?.permissions();
        // perms.set_mode(0o755); // rwxr-xr-x
        // std::fs::set_permissions(&compressed, perms)?;
    }
    Ok(())
}

fn download_file(url: &str, fname: &PathBuf) -> Result<File> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| tauri::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    let bytes = response
        .bytes()
        .map_err(|e| tauri::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    let mut file = File::create(&fname)?;
    file.write_all(&bytes)?;

    Ok(file)
}
