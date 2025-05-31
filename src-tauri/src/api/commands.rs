use std::sync::Mutex;

use crate::api::{AppState, InternalState};
use lib6502::{bus::BusDevice, cpu::RegisterState};

use tauri::{ipc::Channel, State};

use tokio;

#[tauri::command]
pub fn get_registers(state: State<'_, Mutex<AppState>>) -> RegisterState {
    let app_state = state.lock().unwrap();
    app_state.registers
}

#[tauri::command]
pub async fn run_asm(
    state: State<'_, Mutex<AppState>>,
    on_event: Channel<InternalState>,
) -> Result<(), ()> {
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
            on_event
                .send(InternalState::new(
                    app_state.cpu.get_state(),
                    app_state.cpu.get_bus_pins(),
                ))
                .unwrap();
        }

        // Lock released, sleep thread
        tokio::time::sleep(std::time::Duration::from_millis(16)).await;
    }
}

#[tauri::command]
pub fn stop(state: State<'_, Mutex<AppState>>) {
    // Stop a running emulator.
    let mut app_state = state.lock().unwrap();
    app_state.running = false;
}

// Step the cpu forward 1 cycle.
#[tauri::command]
pub fn step(state: State<'_, Mutex<AppState>>, on_event: Channel<InternalState>) {
    let mut app_state = state.lock().unwrap();
    app_state.cpu.cycle();
    on_event
        .send(InternalState::new(
            app_state.cpu.get_state(),
            app_state.cpu.get_bus_pins(),
        ))
        .unwrap();
}

// Use this when the application starts and a new program is loaded to display the ram contents.
#[tauri::command]
pub fn get_nonzero_bytes(state: State<'_, Mutex<AppState>>) -> Vec<(u16, u8)> {
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
