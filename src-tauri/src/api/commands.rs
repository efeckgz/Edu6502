use std::sync::Mutex;

use crate::api::AppState;
use lib6502::cpu::RegisterState;

use tauri::{ipc::Channel, AppHandle, Emitter, State};

use tokio;

#[tauri::command]
pub fn get_registers(state: tauri::State<'_, Mutex<AppState>>) -> RegisterState {
    let app_state = state.lock().unwrap();
    app_state.registers
}

#[tauri::command]
pub async fn run_asm(
    app: AppHandle,
    state: State<'_, Mutex<AppState>>,
    on_event: Channel<RegisterState>,
) -> Result<(), ()> {
    loop {
        // Take the lock only for state update
        {
            let mut app_state = state.lock().unwrap();
            app_state.cpu.cycle(); // run foreva
            on_event.send(app_state.cpu.get_state()).unwrap();
            // app.emit("registers", app_state.cpu.get_state()).unwrap();
            // app_state.registers = app_state.cpu.get_state(); // Update the registers in state to show in frontend
            println!("A: {}", app_state.cpu.a);
        }

        // Lock released, sleep thread
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}
