use app::vsd::{self, core::DownloadState};
use core::time;
use std::{
    error,
    fs::{self, File},
    thread,
    thread::sleep,
    time::Duration,
};
use tauri::{Manager, Window};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}

pub fn download(window: Window) {
    thread::spawn(move || -> () {
        println!("下载进度...");
        loop {
            let t = vsd::core::PROG.lock().unwrap().total;
            let d = vsd::core::PROG.lock().unwrap().downloaded;
        
            if t > 0 && d > 0 && t == d {
                return;
            }
            sleep(time::Duration::from_secs(1));
            println!("total:{}, downloaded:{}", t, d);
            window
                .emit(
                    "downing",
                    Payload {
                        message: format!("{}", d),
                    },
                )
                .unwrap();
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
pub fn set_json(json_str: String) {
    let temp_path = std::env::temp_dir();
    let temp_path = temp_path.to_str().unwrap();
    println!("临时文件路径：{}", temp_path);
    fs::write(format!("{}/tauri.json", temp_path), json_str).unwrap();
}

//下载文件
#[tauri::command]
pub fn downFile(filePath: &str, output: &str) -> String {
    println!("file_path:{}, output:{}", filePath, output);
    args_url(filePath.to_string(), output.to_string());
    "ok".to_string()
}

fn args_url(url: String, output: String) {
    let mut downloader = vsd::core::DownloadState::new_url(url, output).unwrap_or_else(|e| vsd::utils::error(e));
    let segments = downloader.segments().unwrap_or_else(|e| vsd::utils::error(e));

    let prog = downloader.speed.clone();
    thread::spawn(move || {
        loop{
            println!("Download:{},", prog.lock().unwrap().total);
            println!("Download:{},", prog.lock().unwrap().downloaded);
            sleep(time::Duration::from_secs(1));
        }
    });

    downloader.download(&segments, downloader.tempfile()).unwrap_or_else(|e| vsd::utils::error(e));
    downloader.transmux_trancode().unwrap_or_else(|e| vsd::utils::error(e));
}

