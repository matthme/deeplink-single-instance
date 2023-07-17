// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowUrl, WindowBuilder, UserAttentionType, RunEvent, SystemTray, SystemTrayMenu};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {

    tauri_plugin_deep_link::prepare("my-protocol");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {

            let app_handle = app.handle();
            app_handle.plugin(tauri_plugin_single_instance::init(move |_, _argv, _cwd| {
                println!("I am not executed :/");
            }))?;

            if let Err(err) = tauri_plugin_deep_link::register("my-protocol", move |request| {
                println!("Received deeplink request: {}", request);
            }) {
                println!("Error registering the deep link plugin: {:?}", err);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Failed to run tauri app.")
}
