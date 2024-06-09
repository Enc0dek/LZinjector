#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use injector::inject_payload;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Manager, SystemTray};
use tauri::SystemTrayMenu;
use tauri::{State, SystemTrayEvent};

mod filehandler;
mod injector;

use filehandler::get_file;

struct AppState {
    payload_file: Arc<Mutex<Option<PathBuf>>>,
    process: Arc<Mutex<String>>,
    custom_target: Arc<Mutex<bool>>,
}

#[tauri::command]
fn selectfile(file: String, state: State<AppState>) -> Result<i32, String> {
    match get_file(file) {
        Some(payload_file) => {
            *state.payload_file.lock().map_err(|_| "Failed to acquire payload_file lock".to_string())? = Some(payload_file);
            Ok(0)
        }
        None => Err("File Not Found".to_string()),
    }
}

#[tauri::command]
fn get_process(process: String, state: State<AppState>) -> Result<(), String> {
    *state.process.lock().map_err(|_| "Failed to acquire process lock".to_string())? = process;
    Ok(())
}

#[tauri::command]
fn get_custom_target(custom_target: bool, state: State<AppState>) -> Result<(), String> {
    *state.custom_target.lock().map_err(|_| "Failed to acquire custom_target lock".to_string())? = custom_target;
    println!("{}", custom_target);
    Ok(())
}

#[tauri::command]
fn inject(process: String, state: State<AppState>) -> Result<String, String> {
    let payload_file = state.payload_file.lock().map_err(|_| "Failed to acquire payload_file lock".to_string())?;

    if let Some(payload_file) = &*payload_file {
        match inject_payload(process, payload_file) {
            Ok(_) => Ok("Injected".to_string()),
            Err(e) => Err(e),
        }
    } else {
        Err("No DLL Selected".to_string())
    }
}

fn tray_inject(state: State<AppState>) {
    let process = match state.process.lock() {
        Ok(process) => process.clone(),
        Err(_) => return,
    };
    if let Err(_e) = inject(process, state) {
        // handle error if needed
    }
}

#[tauri::command]
fn hide_tray(app: AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.hide();
    }
}

fn open_tray(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        let _ = window.show();
    }
}

fn main() {
    let quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let open = tauri::CustomMenuItem::new("open".to_string(), "Open");
    let injecttray = tauri::CustomMenuItem::new("inject".to_string(), "Inject");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(open)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(injecttray);

    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .manage(AppState {
            process: Arc::new(Mutex::new(String::new())),
            custom_target: Arc::new(Mutex::new(false)),
            payload_file: Arc::new(Mutex::new(None)),
        })
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => open_tray(app),
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                "open" => open_tray(app),
                "inject" => tray_inject(app.state::<AppState>()),
                _ => {}
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            selectfile,
            inject,
            get_process,
            hide_tray,
            get_custom_target
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}