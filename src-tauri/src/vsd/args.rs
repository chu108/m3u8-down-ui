use clap::{ArgEnum, Parser};

#[derive(Debug, Copy, Clone, ArgEnum)]
pub enum Quality {
    Select,
    SD,
    HD,
    FHD,
    UHD,
    UHD4K,
    Max,
}

/// Command line program to download HLS video from websites and m3u8 links.
#[derive(Debug, Clone, Parser)]
#[clap(version, author = "clitic <clitic21@gmail.com>", about)]
pub struct Args {
    /// url | .m3u8 | .m3u
    #[clap(required = true, validator = input_validator)]
    pub input: String,

    /// Path of final downloaded video stream.
    /// For file extension any ffmpeg supported format could be provided.
    /// If playlist contains alternative streams vsd will try to transmux and trancode into single file using ffmpeg.
    #[clap(short, long)]
    pub output: Option<String>,

    /// Base url for all segments.
    /// Usually needed for local m3u8 file.
    #[clap(short, long)]
    pub baseurl: Option<String>,

    /// Automatic selection of some standard resolution streams with highest bandwidth stream variant from master playlist.
    #[clap(short, long, arg_enum, default_value_t = Quality::Select)]
    pub quality: Quality,

    /// Maximum number of threads for parllel downloading of segments.
    /// Number of threads should be in range 1-16 (inclusive).
    #[clap(short, long, default_value_t = 5, validator = threads_validator)]
    pub threads: u8,

    /// Maximum number of retries to download an individual segment.
    #[clap(long, default_value_t = 15)]
    pub retry_count: u8,

    /// Raw style input prompts for old and unsupported terminals.
    #[clap(long)]
    pub raw_prompts: bool,

    /// Resume a download session.
    /// Download session can only be resumed if download session json file is present.
    #[clap(short, long)]
    pub resume: bool,

    /// Download alternative streams such as audio and subtitles streams from master playlist instead of variant video streams.
    #[clap(short, long)]
    pub alternative: bool,

    /// Skip downloading and muxing alternative streams.
    #[clap(short, long)]
    pub skip: bool,

    /// Custom headers for requests.
    /// This option can be used multiple times.
    #[clap(long, multiple_occurrences = true, number_of_values = 2, value_names = &["key", "value"], help_heading = "CLIENT OPTIONS")]
    pub header: Vec<String>, // Vec<Vec<String>> not supported

    /// Update and set custom user agent for requests.
    #[clap(
        long,
        default_value = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36",
        help_heading = "CLIENT OPTIONS"
    )]
    pub user_agent: String,

    /// Set http or https proxy address for requests.
    #[clap(long, validator = proxy_address_validator, help_heading = "CLIENT OPTIONS")]
    pub proxy_address: Option<String>,

    /// Enable cookie store which allows cookies to be stored.
    #[clap(long, help_heading = "CLIENT OPTIONS")]
    pub enable_cookies: bool,

    /// Enable cookie store and fill it with some existing cookies.
    /// Example `--cookies "foo=bar; Domain=yolo.local" https://yolo.local`.
    /// This option can be used multiple times.
    #[clap(long, multiple_occurrences = true, number_of_values = 2, value_names = &["cookies", "url"], help_heading = "CLIENT OPTIONS")]
    pub cookies: Vec<String>, // Vec<Vec<String>> not supported
}

fn input_validator(s: &str) -> Result<(), String> {
    if s.starts_with("https://youtube.com")
        || s.starts_with("https://www.youtube.com")
        || s.starts_with("https://youtu.be")
    {
        Err("Youtube links aren't supported yet".to_owned())
    } else {
        Ok(())
    }
}

fn threads_validator(s: &str) -> Result<(), String> {
    let num_threads: usize = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    if std::ops::RangeInclusive::new(1, 16).contains(&num_threads) {
        Ok(())
    } else {
        Err("Number of threads should be in range `1-16`".to_string())
    }
}

fn proxy_address_validator(s: &str) -> Result<(), String> {
    if s.starts_with("http://") || s.starts_with("https://") {
        Ok(())
    } else {
        Err("Proxy address should start with `http://` or `https://` only".to_string())
    }
}

pub fn parse() -> Args {
    Args::parse()
}
