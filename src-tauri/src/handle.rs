use app::vsd::{self, down, Progress};
use once_cell::sync::Lazy;
use core::time;
use std::{
    fs::{self, File},
    thread,
    thread::sleep,
    sync::{RwLock, mpsc::{self, Sender, Receiver}, Mutex}
};
use tauri::{Manager, Window};

// pub static TASK: Lazy<Mutex<(Sender<Progress>, Receiver<Progress>)>> = Lazy::new(|| {
//     Mutex::new(mpsc::channel())
// });

pub static TASK: Lazy<RwLock<Vec<Progress>>> = Lazy::new(|| {
    let arr = Vec::new();
    RwLock::new(arr)
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
            println!("进度：{}", progress);
            window.emit(
                "downing",
                Payload {
                    message: progress
                },
            ).unwrap();
            if vsd::PROGMAP.read().unwrap().finish() || vsd::PROGMAP.read().unwrap().err != "" {
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
            //channel方式
            // let task:Progress;
            // {
            //     task = TASK.lock().unwrap().1.recv().unwrap();
            // }
            // println!("开始下载,Url:{}, Output:{}", task.url.clone(), task.output.clone());
            // downWatch(window.clone());
            // match down(task.url.clone(), task.output.clone()) {
            //     Ok(_) => println!("下载成功"),
            //     Err(e) => {
            //         vsd::PROGMAP.write().unwrap().err = format!("{}", e);
            //     }
            // }
            
            //数组方式
            if TASK.read().unwrap().len() > 0 {
                let task = TASK.write().unwrap().remove(0);
                println!("开始下载,Url:{}, Output:{}", task.url.clone(), task.output.clone());
                downWatch(window.clone());
                match down(task.url.clone(), task.output.clone()) {
                    Ok(_) => println!("下载成功"),
                    Err(e) => {
                        vsd::PROGMAP.write().unwrap().err = format!("{}", e);
                    }
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
    TASK.write().unwrap().push(prog);
    // TASK.lock().unwrap().0.clone().send(prog);
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