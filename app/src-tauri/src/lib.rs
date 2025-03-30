mod commands;
use commands::{
    ProcessHandle, start_listening_glove
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ProcessHandle::new())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_listening_glove])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
