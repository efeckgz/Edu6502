use std::sync::Mutex;

use crate::api::Devices;
use lib6502::{
    bus::BusDevice,
    cpu::{Cpu, RegisterState},
};

// Type alias to help typing
type AppCpu = Mutex<Cpu<Devices, 1>>;

static ROM: [u8; include_bytes!("a.out").len()] = *include_bytes!("a.out");

#[tauri::command]
pub fn get_registers(state: tauri::State<'_, Mutex<Cpu<Devices, 1>>>) -> RegisterState {
    let cpu = state.lock().unwrap();
    cpu.get_state()
}

#[tauri::command]
pub fn run_asm(state: tauri::State<'_, AppCpu>) {
    let mut cpu = state.lock().unwrap();

    // DO NOT DO THIS THIS IS ONLY TO TEST DO NOT LOAD PROGRAMS BY DOING BUS WRITES
    for (addr, byte) in ROM.iter().enumerate() {
        cpu.bus.write(addr as u16, *byte);
    }

    loop {
        cpu.cycle(); // run foreva
        println!("A: {}", cpu.a);
    }
}
