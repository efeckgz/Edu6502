use std::sync::Mutex;

use crate::api::Devices;
use lib6502::cpu::{Cpu, RegisterState};
use tauri::State;

#[tauri::command]
pub fn get_registers(state: State<'_, Mutex<Cpu<Devices, 1>>>) -> RegisterState {
    let cpu = state.lock().unwrap();
    cpu.get_state()
}
