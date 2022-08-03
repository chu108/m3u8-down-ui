use tauri::{Manager};
use tauri::{AppHandle, SystemTrayEvent};

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    match event {  //监听托盘事件
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => {
                println!("进程退出3");
                std::process::exit(0);
            }
            "hide" => {
                let window = app.get_window("main").unwrap();
                window.hide().unwrap();
            }
            "show" => {
              let window = app.get_window("main").unwrap();
              window.show().unwrap();
          }
            _ => {}
        },
        _ => {}
    }
}
