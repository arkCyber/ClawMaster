// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Import the library module
use clawmaster_tauri::init_app;

fn main() {
    init_app()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
