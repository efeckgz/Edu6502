use std::{process::Command, sync::Mutex};

use crate::api::{stream_cpu_state, AppState, InternalState, ROM};
use lib6502::bus::BusDevice;

use tauri::{ipc::Channel, AppHandle, Error, Manager, Result, State};

use tokio;

use super::Devices;

#[tauri::command]
pub async fn run_asm(
    state: State<'_, Mutex<AppState>>,
    chan: Channel<InternalState>,
) -> Result<()> {
    // Set running to true in a separate block so that the lock is dropped.
    {
        let mut app_state = state.lock().unwrap();
        app_state.running = true;
    }

    loop {
        // Take the lock only for state update
        {
            let mut app_state = state.lock().unwrap();
            if !app_state.running {
                return Ok(());
            }

            app_state.cpu.cycle();
            stream_cpu_state(&app_state.cpu, &chan)?;
        }

        // Lock released, sleep thread
        tokio::time::sleep(std::time::Duration::from_millis(32)).await;
    }
}

#[tauri::command]
pub fn stop(state: State<Mutex<AppState>>) {
    // Stop a running emulator.
    let mut app_state = state.lock().unwrap();
    app_state.running = false;
}

// Step the cpu forward 1 cycle.
#[tauri::command]
pub fn step(state: State<Mutex<AppState>>, chan: Channel<InternalState>) -> Result<()> {
    let mut app_state = state.lock().unwrap();
    app_state.cpu.cycle();
    stream_cpu_state(&app_state.cpu, &chan)?;
    Ok(())
}

#[tauri::command]
pub fn reset(state: State<Mutex<AppState>>, chan: Channel<InternalState>) -> Result<()> {
    let mut app_state = state.lock().unwrap();
    app_state.cpu.reset(); // Reset the cpu

    let ram = app_state.cpu.bus.borrow_device_mut(1).unwrap();
    let Devices::Ram(r) = ram;
    r.reset(); // Reset the ram
    r.load_program(&ROM); // Load the current program back

    // Stream the cpu state. The frontend will call get_nonzero_bytes to get ram state.
    stream_cpu_state(&app_state.cpu, &chan)?;
    Ok(())
}

// Use this when the application starts and a new program is loaded to display the ram contents.
#[tauri::command]
pub fn get_nonzero_bytes(state: State<Mutex<AppState>>) -> Vec<(u16, u8)> {
    let mut app_state = state.lock().unwrap();
    let mut result = vec![];

    for i in 0..65535 {
        let val = app_state.cpu.bus.read(i);
        if val != 0 {
            result.push((i, val));
        }
    }
    result
}

#[tauri::command]
pub fn assemble_and_load(app_handle: AppHandle, program: &str) -> Result<String> {
    // Get the assembler dir
    let mut dir = app_handle.path().app_data_dir()?;
    dir.push("assembler");

    // Write the program into a file
    dir.push("temp.s");
    std::fs::write(&dir, program)?;

    // Invoke the assembler
    let s_dir = dir.clone();
    dir.pop();
    dir.push("vasm6502_oldstyle");
    let vasm_dir = dir.clone();
    dir.pop();
    dir.push("temp.out");
    let out_dir = dir.clone();

    let output = Command::new(vasm_dir)
        .arg("-Fbin")
        .arg(s_dir)
        .arg("-o")
        .arg(out_dir)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(Error::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", stderr),
        )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}
