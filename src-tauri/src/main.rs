#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod event;
mod handle;
mod menu;

fn main() {
    tauri::Builder::default()
        .on_page_load(move |_window, _payload| {
            println!("on_page_load........................");
            handle::up_watch(_window);
        })
        .setup(move |_app| {
            println!("setup........................");
            Ok(())
        })
        .system_tray(menu::create_menu())
        .on_system_tray_event(event::handler)
        .invoke_handler(tauri::generate_handler![
            handle::greet,
            handle::down_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
