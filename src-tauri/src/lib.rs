pub mod audio_manager;
mod commands;
pub mod key_listener;
mod mechvibes;
mod packs;
mod state;

use state::AppState;
use std::{sync::mpsc, thread};
use tauri::Manager;

pub const APP_NAME: &str = "WhisperKeys";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = mpsc::channel();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState::default().unwrap());

            thread::spawn(move || {
                key_listener::start(tx);
            });

            let handle = app.handle().clone();
            thread::spawn(move || audio_manager::start(rx, handle));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_available_packs,
            commands::select_pack,
            commands::toggle_mute,
            commands::set_volume,
            commands::refresh_available_packs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
