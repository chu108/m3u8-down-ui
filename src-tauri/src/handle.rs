use app::vsd::{self, down, Progress};
use once_cell::sync::Lazy;
use core::time;
use std::{
    fs::{self, File},
    thread,
    thread::sleep,
    sync::RwLock, collections::HashMap
};
use tauri::{Manager, Window};

pub static TASK: Lazy<RwLock<HashMap<String, Progress>>> = Lazy::new(|| {
    let prog = HashMap::new();
    RwLock::new(prog)
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
            let progress = vsd::PROGMAP.read().unwrap().to_json();
            // println!("进度：{}", progress);
            window
                .emit(
                    "downing",
                    Payload {
                        message: progress
                    },
                )
                .unwrap();
            if vsd::PROGMAP.read().unwrap().finish() || vsd::PROGMAP.read().unwrap().err != "" {
                //队列中的数据
                if TASK.write().unwrap().contains_key(&vsd::PROGMAP.read().unwrap().url) {
                    TASK.write().unwrap().remove(&vsd::PROGMAP.read().unwrap().url).unwrap();
                }
                //清空进度
                vsd::PROGMAP.write().unwrap().clear();
                if vsd::PROGMAP.read().unwrap().err != "" {
                    println!("下载失败，{}", vsd::PROGMAP.read().unwrap().err);
                } else {
                    println!("下载完成");
                }
                
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
            if TASK.read().unwrap().len() > 0 {
                for (key, task) in TASK.read().unwrap().iter() {
                    if task.total == 0 && vsd::PROGMAP.read().unwrap().url == "" {
                        println!("开始下载,Url:{}, Output:{}", task.url.clone(), task.output.clone());
                        downWatch(window.clone());
                        match down(task.url.clone(), task.output.clone()) {
                            Ok(_) => println!("下载成功"),
                            Err(e) => {
                                vsd::PROGMAP.write().unwrap().err = format!("{}", e);
                            }
                        }
                    }
                    println!("-----------循环次数:{}", key);
                    //测试----------------------------
                    // downWatch(window.clone());
                    // println!("开始下载,Url:{}, Output:{}", task.url.clone(), task.output.clone());
                    // vsd::PROGMAP.write().unwrap().url = "http://1257120875.vod2.myqcloud.com/0ef121cdvodtransgzp1257120875/3055695e5285890780828799271/v.f230.m3u8".to_string();
                    // vsd::PROGMAP.write().unwrap().total = 100;
                    // vsd::PROGMAP.write().unwrap().downloaded = 100;
                    // vsd::PROGMAP.write().unwrap().err = format!("下载完成{}", key);
                }
            }
            sleep(time::Duration::from_secs(2));
        }
    });
}

//下载文件
#[tauri::command]
pub fn downFile(filePath: &str, output: &str) -> String {
    let mut prog = Progress::new();
    prog.set_url_out(filePath.to_string(), output.to_string());
    TASK.write().unwrap().insert(filePath.to_string(), prog);
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