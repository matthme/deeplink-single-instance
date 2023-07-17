// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowUrl, WindowBuilder};

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
            app_handle.plugin(tauri_plugin_single_instance::init(move |app, _argv, _cwd| {
                let main_window = app.get_window("deeplink-single-instance");
                if let Some(window) = main_window {
                    window.show().unwrap();
                    window.unminimize().unwrap();
                    window.set_focus().unwrap();
                } else {
                    WindowBuilder::new(app, "deeplink-single-instance", WindowUrl::App("index.html".into()))
                        .title("deeplink-single-instance")
                        .inner_size(1000.0, 700.0)
                        .center()
                        .build()
                        .unwrap();
                }
            }))?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
