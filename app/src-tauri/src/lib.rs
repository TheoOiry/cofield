mod commands;
use commands::{ProcessConfig, ProcessHandle};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(ProcessHandle::new())
        .manage(ProcessConfig::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::start_listening_glove,
            commands::stop_listening_glove,
            commands::set_aggregation_size,
            commands::set_keyboard_emulation_config,
            commands::set_output_raw_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
