use std::{thread::sleep, time::Duration, fs::File, path::Path};

use tauri::{Manager, Window};

pub fn update(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    println!("setup。。。。。。。。。。。");

    // for (win_name, win) in app.windows() {
    //     println!("{}, {}", win_name, win.label());
    // }

    // let handle = app.handle();
    // std::thread::spawn(move || -> () {
    //     sleep(Duration::from_secs(1));
    //     let _local_window = tauri::WindowBuilder::new(
    //                 &handle,
    //                 "local",
    //                 tauri::WindowUrl::External("https://developer.huawei.com/consumer/cn/doc/development/app/agc-help-releasenotes-0000001150208779#section20301321537".parse().unwrap()),
    //             )
    //             .maximized(true)
    //             // .inner_size(400.0, 600.0)
    //             .title("版本更新-公告")
    //             .build().unwrap();
    // });
    Ok(())
}

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
