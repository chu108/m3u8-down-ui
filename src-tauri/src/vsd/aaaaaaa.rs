use std::{
    thread::{self, sleep},
    time,
};

use kdam::term::Colorizer;
use vsd::core::PROG;

fn aaaa() {
    // let mut downloader = vsd::core::DownloadState::new().unwrap_or_else(|e| error(e));
    // let segments = downloader.segments().unwrap_or_else(|e| error(e));
    // downloader.download(&segments, downloader.tempfile()).unwrap_or_else(|e| error(e));
    // downloader.transmux_trancode().unwrap_or_else(|e| error(e));

    thread::spawn(|| -> () {
        println!("下载进度...");
        loop {
            let t = PROG.lock().unwrap().total;
            let d = PROG.lock().unwrap().downloaded;
            println!("total:{}, downloaded:{}", t, d);
            if t > 0 && d > 0 && t == d {
                return;
            }
            sleep(time::Duration::from_secs(1));
        }
    });

    args_url(
        "https://cdn.aqd-tv.com/video/8c7c4dd5/playlist/8c7c4dd5.m3u8".to_string(),
        "./22.mp4".to_string(),
    );
}

fn args_url(url: String, output: String) {
    let mut downloader =
        vsd::core::DownloadState::new_url(url, output).unwrap_or_else(|e| error(e));
    let segments = downloader.segments().unwrap_or_else(|e| error(e));
    downloader
        .download(&segments, downloader.tempfile())
        .unwrap_or_else(|e| error(e));
    downloader.transmux_trancode().unwrap_or_else(|e| error(e));
}
