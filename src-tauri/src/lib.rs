use tauri::{Manager, Result};

mod api;
use api::commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            check_install_assembler(app)?;
            let state = api::initialize();
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::run_asm,
            commands::stop,
            commands::step,
            commands::reset,
            commands::get_nonzero_bytes,
            commands::assemble_and_load,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn check_install_assembler(app: &tauri::App) -> Result<()> {
    let mut dir = app.path().app_data_dir()?;
    dir.push("assembler");
    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    dir.push("vasm6502_oldstyle");
    if dir.is_file() {
        // println!("Yay!")
    }

    Ok(())
}
