use std::sync::Mutex;

use crate::api::AppState;
use lib6502::cpu::RegisterState;

use tauri::{ipc::Channel, State};

use tokio;

#[tauri::command]
pub fn get_registers(state: tauri::State<'_, Mutex<AppState>>) -> RegisterState {
    let app_state = state.lock().unwrap();
    app_state.registers
}

#[tauri::command]
pub async fn run_asm(
    state: State<'_, Mutex<AppState>>,
    on_event: Channel<RegisterState>,
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
            on_event.send(app_state.cpu.get_state()).unwrap();
        }

        // Lock released, sleep thread
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

#[tauri::command]
pub fn stop(state: State<'_, Mutex<AppState>>) {
    // Stop a running emulator.
    let mut app_state = state.lock().unwrap();
    app_state.running = false;
}
