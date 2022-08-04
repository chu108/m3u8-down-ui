pub mod args;
pub mod downloader;
pub mod merger;
pub mod decrypt;
pub mod utils;
pub mod core;
pub mod progress;
pub mod parse;

use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};
use std::sync::{RwLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub url: String,
    pub total: usize,
    pub downloaded: usize,
    pub err:String,
    pub output: String,
}

impl Progress{
    pub fn new() -> Self {
        Progress{url:"".to_string(), total:0, downloaded:0,err:"".to_string(), output:"".to_string()}
    }

    pub fn init(&mut self, url: String, total: usize, downloaded: usize) {
        self.url = url;
        self.total = total;
        self.downloaded = downloaded;
        self.err = String::from("");
    }

    pub fn set_url_out(&mut self, url: String, output: String) {
        self.url = url;
        self.output = output;
    }
    
    pub fn set_download(&mut self, download: usize) {
        self.downloaded += download;
    }

    pub fn set_err(&mut self, err: String) {
        self.err = err;
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn clear(&mut self) {
        self.url = "".to_string();
        self.total = 0;
        self.downloaded = 0;
        self.err = "".to_string();
        self.output = "".to_string();
    }

    pub fn finish(&self) -> bool {
        self.total > 0 && self.downloaded > 0 && self.total == self.downloaded
    }
}

pub static PROGMAP: Lazy<RwLock<Progress>> = Lazy::new(|| {
    let prog = Progress::new();
    RwLock::new(prog)
});

pub fn down(url: String, output: String) -> Result<(), anyhow::Error> {
    let mut downloader = core::DownloadState::new_url(url, output)?;
    let segments = downloader.segments()?;
    downloader.download(&segments, downloader.tempfile())?;
    downloader.transmux_trancode()?;
    println!("down finish。。。。。");
    Ok(())
}