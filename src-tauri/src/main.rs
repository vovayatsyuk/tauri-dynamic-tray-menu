// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            std::thread::spawn(move || {
                let mut i = 0;
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(1000));

                    i += 1;
                    let tray_menu = SystemTrayMenu::new().add_item(
                        CustomMenuItem::new("item".to_string(), format!("Item {}", i))
                    );

                    handle.tray_handle().set_menu(tray_menu).unwrap();
                }
            });

            Ok(())
        })
        .system_tray(system_tray)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
