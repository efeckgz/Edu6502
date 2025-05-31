use tauri::Manager;

mod api;
use api::commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = api::initialize();
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_registers,
            commands::run_asm,
            commands::stop,
            commands::step,
            commands::get_nonzero_bytes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
