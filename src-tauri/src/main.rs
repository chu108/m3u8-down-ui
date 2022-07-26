#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod event;
mod handle;
mod menu;
mod window;

fn main() {
    tauri::Builder::default()
        .on_page_load(move |_window, _payload| {
            println!("on_page_load........................");
            // handle::download(_window);
        })
        .setup(move |app| {
            println!("setup........................");
            Ok(())
        })
        .system_tray(menu::create_menu())
        .on_system_tray_event(event::handler)
        .invoke_handler(tauri::generate_handler![
            handle::greet,
            // handle::download,
            handle::close_splashscreen,
            handle::update_maximized,
            handle::downFile
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
