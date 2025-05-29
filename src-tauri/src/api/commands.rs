use std::sync::Mutex;

use crate::api::AppState;
use lib6502::cpu::RegisterState;

use tokio;

#[tauri::command]
pub fn get_registers(state: tauri::State<'_, Mutex<AppState>>) -> RegisterState {
    let app_state = state.lock().unwrap();
    app_state.registers
}

#[tauri::command]
pub async fn run_asm(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), ()> {
    loop {
        // Take the lock only for state update
        {
            let mut app_state = state.lock().unwrap();
            app_state.cpu.cycle(); // run foreva
            app_state.registers = app_state.cpu.get_state(); // Update the registers in state to show in frontend
            println!("A: {}", app_state.cpu.a);
        }

        // Lock released, sleep thread
        tokio::time::sleep(std::time::Duration::from_millis(32)).await; // Run at 30Hz
    }
}
