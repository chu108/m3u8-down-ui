use std::{fs::File, path::Path};

use tauri::{Manager, Window};

pub fn check_status(app: &mut tauri::App, status: bool) {
    if status {
        app.get_window("main").unwrap().maximize();
    }
}

pub fn vli_status(win: &Window, status: bool) {
    if status {
        win.maximize();
    }
}

pub fn get_status() -> bool {
    let tmp = std::env::temp_dir();
    let path = format!("{}/tauri.json", tmp.to_str().unwrap());
    if !Path::new(path.as_str()).exists() {
        return false;
    }
    let f = File::open(path).unwrap();
    let v: serde_json::Value = serde_json::from_reader(f).unwrap();
    if let Some(_id) = v["id"].as_i64() {
        return true;
    }
    false
}
