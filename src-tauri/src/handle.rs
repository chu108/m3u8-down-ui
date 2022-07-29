use app::vsd::{self, down, Progress};
use once_cell::sync::Lazy;
use core::time;
use std::{
    fs::{self, File},
    thread,
    thread::sleep,
    sync::Mutex, collections::HashMap
};
use tauri::{Manager, Window};

pub static TASK: Lazy<Mutex<HashMap<String, Progress>>> = Lazy::new(|| {
    let prog = HashMap::new();
    Mutex::new(prog)
});

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}

pub fn downWatch(window: Window) {
    thread::spawn(move || -> () {
        println!("下载进度...");
        loop {
            let progress = vsd::PROGMAP.lock().unwrap().to_json();
            println!("进度：{}", progress);
            window
                .emit(
                    "downing",
                    Payload {
                        message: progress
                    },
                )
                .unwrap();
            if vsd::PROGMAP.lock().unwrap().finish() {
                //队列中的数据
                TASK.lock().unwrap().remove(&vsd::PROGMAP.lock().unwrap().url).unwrap();
                //清空进度
                vsd::PROGMAP.lock().unwrap().clear();
                println!("下载完成，线程退出。。。");
                return;
            }
            sleep(time::Duration::from_secs(1));
        }
    });
}

//监听下载任务
pub fn upWatch(window: Window) {
    thread::spawn(move || -> () {
        println!("开始监听下载任务...");
        loop {
            for (key, task) in TASK.lock().unwrap().iter() {
                if task.total == 0 && vsd::PROGMAP.lock().unwrap().url == "" {
                    println!("开始下载:{}。。。", key);
                    down(task.url.clone(), task.output.clone());
                    downWatch(window.clone());
                    sleep(time::Duration::from_secs(5));
                }
            }
        }
    });
}

//下载文件
#[tauri::command]
pub fn downFile(filePath: &str, output: &str) -> String {
    let mut prog = Progress::new();
    prog.set_url_out(filePath.to_string(), output.to_string());
    TASK.lock().unwrap().insert(filePath.to_string(), prog);
    println!("添加任务到队列：{}", filePath);
    "ok".to_string()
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