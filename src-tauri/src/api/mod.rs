pub mod commands;

use flate2::read::GzDecoder;
use lib6502::bus::Bus;
use lib6502::cpu::Cpu;
use reqwest;
use std::process::Command;
use std::{fs::File, io::Write, os::unix::fs::PermissionsExt, path::PathBuf, sync::Mutex};
use tar::Archive;
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
    let dest = dir.join("vasm.tar.gz");
    download_file(url, &dest)?;

    // Unpack the tarball.
    decompress(&dest, &dir)?;
    let build_dir = dir.join("vasm");

    // Build the assembler.
    let mut cmd = if cfg!(windows) {
        todo!("Building the assembler on windows")
    } else {
        Command::new("make")
    };

    let output = cmd
        .current_dir(&build_dir)
        .arg("CPU=6502")
        .arg("SYNTAX=oldstyle")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(tauri::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", stderr),
        )));
    }

    // Set executable permissions on Unix systems.
    #[cfg(unix)]
    {
        let bin_name = build_dir.join("vasm6502_oldstyle");
        let mut perms = std::fs::metadata(&bin_name)?.permissions();
        perms.set_mode(0o755); // rwxr-xr-x
        std::fs::set_permissions(&bin_name, perms)?;
    }

    // Move the files out of build directory.
    std::fs::rename(
        build_dir.join("vasm6502_oldstyle"),
        dir.join("vasm6502_oldstyle"),
    )?;

    std::fs::rename(build_dir.join("vobjdump"), dir.join("vobjdump"))?;

    // Reamove the tarball and build directory
    std::fs::remove_file(dir.join("vasm.tar.gz"))?;
    std::fs::remove_dir_all(dir.join("vasm"))?;

    Ok(())
}

fn download_file(url: &str, fname: &PathBuf) -> Result<()> {
    let response = reqwest::blocking::get(url)
        .map_err(|e| tauri::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    let bytes = response
        .bytes()
        .map_err(|e| tauri::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    let mut file = File::create(&fname)?;
    file.write_all(&bytes)?;

    Ok(())
}

fn decompress(src: &PathBuf, out: &PathBuf) -> Result<()> {
    let tar_gz = File::open(src)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(out)?;
    Ok(())
}
