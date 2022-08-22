use app::vsd::{self, down, Progress, args::Args};
use once_cell::sync::Lazy;
use core::time;
use std::{
    thread,
    thread::sleep,
    sync::{RwLock}
};
use tauri::{Window};

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

pub fn down_watch(window: Window) {
    thread::spawn(move || -> () {
        println!("下载进度...");
        loop {
            let progress = vsd::PROGMAP.read().unwrap().to_json();
            // println!("进度：{}", progress);
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

pub fn task_count(window: Window) {
    thread::spawn(move || -> () {
        println!("监听等待任务数...");
        loop {
            window.emit(
                "task_count",
                Payload {
                    message: TASK.read().unwrap().len().to_string()
                },
            ).unwrap();
            sleep(time::Duration::from_secs(1));
        }
    });
}

//监听下载任务
pub fn up_watch(window: Window) {
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
                down_watch(window.clone());
                let mut arg = Args::new();
                arg.input = task.url;
                arg.output = Some(task.output);
                arg.threads = task.threads;
                arg.proxy_address = task.proxy_address;
                match down(arg) {
                    Ok(_) => println!("下载成功"),
                    Err(e) => {
                        vsd::PROGMAP.write().unwrap().err = format!("{}", e);
                    }
                }
            }
            sleep(time::Duration::from_secs(1));
        }
    });
}

//下载文件
#[tauri::command]
pub fn down_file(
    file_path: &str, 
    output: &str,
    thread: u8,
    proxy_address: String
) -> String {
    let mut prog = Progress::new();
    prog.url = file_path.to_string();
    prog.output = output.to_string();
    prog.threads = thread;
    if !proxy_address.is_empty() {
        prog.proxy_address = Some(proxy_address);
    }
    prog.set_url_out(file_path.to_string(), output.to_string());
    TASK.write().unwrap().push(prog);
    // TASK.lock().unwrap().0.clone().send(prog);
    println!("添加任务到队列：{}", file_path);
    "ok".to_string()
}