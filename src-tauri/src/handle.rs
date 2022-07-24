use tauri::{Window, Manager};
use std::{thread, thread::sleep, time::Duration, fs::{self, File}};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
  pub message: String,
}

#[tauri::command]
pub fn download(name: &str, window: Window) {
    println!("开始下载文件：{}...", name);
    thread::spawn(move||{
      for i in 1..=100 {
        sleep(Duration::from_millis(100));
        println!("{}", i);
        window.emit("downing", Payload{message:format!("{}", i)}).unwrap();
      }
    });
}

//开机启动画面
#[tauri::command]
pub fn close_splashscreen(window: tauri::Window) {
  // sleep(Duration::from_secs(2));
  // Close splashscreen
  if let Some(splashscreen) = window.get_window("splashscreen") {
    splashscreen.close().unwrap();
  }
  // Show main window
  window.get_window("main").unwrap().show().unwrap();
}

//设置窗口最大化
#[tauri::command]
pub fn update_maximized(window: tauri::Window) {
  // sleep(Duration::from_secs(2));
  // let _ = window.maximize();
}

//设置窗口最大化
#[tauri::command]
pub fn check_status(window: tauri::Window) {
  let temp_path = std::env::temp_dir();
  let temp_path = temp_path.to_str().unwrap();
  let f = File::open(format!("{}/tauri.json", temp_path)).unwrap();
  let v: serde_json::Value = serde_json::from_reader(f).unwrap();
  if let Some(_id) = v["id"].as_i64() {
    println!("读取文件成功，窗口最大化");
    let _ = window.maximize();
  }
}

//存储json
#[tauri::command]
pub fn set_json(json_str:String) {
  let temp_path = std::env::temp_dir();
  let temp_path = temp_path.to_str().unwrap();
  println!("临时文件路径：{}", temp_path);
  fs::write(format!("{}/tauri.json", temp_path), json_str).unwrap();
}
