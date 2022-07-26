use std::sync::{Arc, Mutex};

use anyhow::{anyhow, bail, Context, Result};
use kdam::prelude::BarMethods;
use kdam::term::Colorizer;

use crate::vsd::merger::BinarySequence;
use crate::vsd::parse;
use crate::vsd::progress::{DownloadProgress, StreamData};
use crate::vsd::utils::*;
use once_cell::sync::Lazy;

pub struct Progress {
    pub total: usize,
    pub downloaded: usize,
}

pub static PROG: Lazy<Mutex<Progress>> = Lazy::new(|| {
    let prog = Progress{total: 0, downloaded: 0};
    Mutex::new(prog)
});

pub struct DownloadState {
    args: crate::vsd::args::Args,
    downloader: crate::vsd::downloader::Downloader,
    pub progress: DownloadProgress,
    pub speed: Arc<Mutex<Progress>>,
}

impl DownloadState {
    pub fn new_url(url:String, output:String) -> Result<Self> {
        let args = crate::vsd::args::Args{
            input:url,
            output:Some(output),
            baseurl:None,
            quality:crate::vsd::args::Quality::Select,
            threads:5,
            retry_count:15,
            raw_prompts:false,
            resume:false,
            alternative:false,
            skip:false,
            header:vec![],
            user_agent:"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36".to_string(),
            proxy_address:None,
            enable_cookies:false,
            cookies:vec![],
        };
        Self::new_obj(args)
    }
    
    pub fn new() -> Result<Self> {
        let args = crate::vsd::args::parse();
        Self::new_obj(args)
    }

    fn new_obj(args: crate::vsd::args::Args) -> Result<Self> {
        let downloader = crate::vsd::downloader::Downloader::new(
            &args.user_agent,
            &args.header,
            &args.proxy_address,
            args.enable_cookies,
            &args.cookies,
        )
        .context("Couldn't create reqwest client.")?;

        if let Some(output) = &args.output {
            if !output.ends_with(".ts") {
                check_ffmpeg("the given output doesn't have .ts file extension")?
            }
        }

        Ok(Self {
            args,
            downloader,
            progress: DownloadProgress::new_empty(),
            speed: Arc::new(Mutex::new(Progress{total:0, downloaded:0}))
        })
    }

    fn get_url(&self, uri: &str) -> Result<String> {
        if uri.starts_with("http") {
            Ok(uri.to_owned())
        } else {
            if let Some(baseurl) = &self.args.baseurl {
                Ok(reqwest::Url::parse(baseurl)?.join(&uri)?.to_string())
            } else {
                if !self.args.input.starts_with("http") {
                    bail!(
                        "Non HTTP input should have {} set explicitly.",
                        "--baseurl".colorize("bold green")
                    )
                }

                Ok(reqwest::Url::parse(&self.args.input)?
                    .join(&uri)?
                    .to_string())
            }
        }
    }

    pub fn tempfile(&self) -> String {
        let path = if let Some(output) = self
            .args
            .input
            .split("?")
            .next()
            .unwrap()
            .split("/")
            .find(|x| x.ends_with(".m3u8"))
        {
            if output.ends_with(".ts.m3u8") {
                output.trim_end_matches(".m3u8").to_owned()
            } else {
                replace_ext(output, "ts")
            }
        } else {
            "merged.ts".to_owned()
        };

        if std::path::Path::new(&path).exists() && !self.args.resume {
            let stemed_path = std::path::Path::new(&path)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            for i in 1..9999 {
                let core_file_copy = format!("{} ({}).ts", stemed_path, i);

                if !std::path::Path::new(&core_file_copy).exists() {
                    return core_file_copy;
                }
            }
        }
        path
    }

    fn scrape_website(&mut self) -> Result<()> {
        println!("Scraping website for HLS and Dash links.");
        let resp = self.downloader.get(&self.args.input)?;
        let links = crate::vsd::utils::find_hls_dash_links(&resp.text()?);

        match links.len() {
            0 => bail!(
                "No links found on website source.\n\n\
                {} Consider using {} flag and then \
                run the command with same arguments by replacing the {} with captured m3u8 url.\n\n\
                Suppose first command captures https://streaming.site/video_001/master.m3u8\n\
                $ vsd --capture {}\n\
                $ vsd https://streaming.site/video_001/master.m3u8 \n\n\
                {} Consider using {} flag \
                and then run the command with saved .m3u8 file as {}. \n\n\
                Suppose first command saves master.m3u8\n\
                $ vsd --collect --build {}\n\
                $ vsd master.m3u8",
                "TRY THIS:".colorize("yellow"),
                "--capture".colorize("bold green"),
                "INPUT".colorize("bold green"),
                self.args.input,
                "OR THIS:".colorize("yellow"),
                "--collect --build".colorize("bold green"),
                "INPUT".colorize("bold green"),
                self.args.input,
            ),
            1 => {
                self.args.input = links[0].clone();
                println!("Found one link {}", &links[0]);
            }
            _ => {
                let mut elinks = vec![];
                for (i, link) in links.iter().enumerate() {
                    elinks.push(format!("{:#2}) {}", i + 1, link));
                }
                let index = select(
                    "Select one link:".to_string(),
                    &elinks,
                    self.args.raw_prompts.clone(),
                )?;
                self.args.input = links[index].clone();
            }
        }

        if self.args.input.ends_with(".mpd") {
            bail!("Dash streams are not supported.")
        }
        
        Ok(())
    }

    fn download_alternative(&mut self, master: &m3u8_rs::MasterPlaylist) -> Result<()> {
        let stream_input = self.args.input.clone();
        let audio_tempfile = format!(
            "{}_audio.ts",
            self.progress.stream.file.trim_end_matches(".ts")
        );
        let subtitle_tempfile = format!(
            "{}_subtitles.vtt",
            self.progress.stream.file.trim_end_matches(".ts")
        );
        let subtitle_output = format!(
            "{}_subtitles.srt",
            self.progress.stream.file.trim_end_matches(".ts")
        );

        for alternative in &master.alternatives {
            self.args.input = stream_input.clone();

            match alternative.media_type {
                m3u8_rs::AlternativeMediaType::Audio => {
                    if alternative.autoselect {
                        if let Some(uri) = &alternative.uri {
                            println!("{} audio stream.", "Downloading".colorize("bold green"));
                            check_ffmpeg("audio stream needs to muxed with video stream")?;
                            self.args.input = self.get_url(uri)?;
                            self.progress.current("audio");
                            self.progress.audio =
                                Some(StreamData::new(&self.args.input, &audio_tempfile));

                            let content = self.downloader.get_bytes(&self.args.input)?;
                            match m3u8_rs::parse_playlist_res(&content).map_err(|_| {
                                anyhow!("Couldn't parse {} playlist.", self.args.input)
                            })? {
                                m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                                    self.download(&meadia.segments, audio_tempfile.clone())?;
                                }
                                _ => (),
                            }
                        }
                    }
                }

                m3u8_rs::AlternativeMediaType::Subtitles
                | m3u8_rs::AlternativeMediaType::ClosedCaptions => {
                    if alternative.autoselect {
                        if let Some(uri) = &alternative.uri {
                            println!("{} subtitles stream.", "Downloading".colorize("bold green"));
                            check_ffmpeg("subtitles stream needs to muxed with video stream")?;
                            self.args.input = self.get_url(uri)?;
                            self.progress.current("subtitle");
                            self.progress.subtitle =
                                Some(StreamData::new(&self.args.input, &subtitle_tempfile));

                            let content = self.downloader.get_bytes(&self.args.input)?;
                            match m3u8_rs::parse_playlist_res(&content).map_err(|_| {
                                anyhow!("Couldn't parse {} playlist.", self.args.input)
                            })? {
                                m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                                    self.download(&meadia.segments, subtitle_tempfile.clone())?;
                                }
                                _ => (),
                            }

                            if std::path::Path::new(&subtitle_output).exists() {
                                std::fs::remove_file(&subtitle_output)?;
                            }

                            println!(
                                "Executing {}",
                                [
                                    "ffmpeg",
                                    "-i",
                                    &subtitle_tempfile,
                                    "-c",
                                    "copy",
                                    &subtitle_output
                                ]
                                .join(" ")
                                .colorize("cyan")
                            );

                            let code = std::process::Command::new("ffmpeg")
                                .args(["-i", &subtitle_tempfile, "-c", "copy", &subtitle_output])
                                .stderr(std::process::Stdio::null())
                                .spawn()?
                                .wait()?;

                            if !code.success() {
                                bail!("FFMPEG exited with code {}.", code.code().unwrap_or(1))
                            }

                            std::fs::remove_file(&subtitle_tempfile)?;

                            if let Some(subtitle) = &mut self.progress.subtitle {
                                subtitle.file = subtitle_tempfile.clone();
                            }
                        }
                    }
                }

                _ => (),
            }
        }

        self.args.input = stream_input;
        self.progress.current("stream");
        Ok(())
    }

    pub fn segments(&mut self) -> Result<Vec<m3u8_rs::MediaSegment>> {
        let content = if self.args.input.starts_with("http") {
            if !self
                .args
                .input
                .split("?")
                .next()
                .unwrap()
                .ends_with(".m3u8")
            {
                self.scrape_website()?;
            }

            self.downloader.get_bytes(&self.args.input)?
        } else {
            std::fs::read_to_string(&self.args.input)?
                .as_bytes()
                .to_vec()
        };

        self.parse_playlist(content)
    }

    pub fn segments_url(&mut self, url:String, output:String) -> Result<Vec<m3u8_rs::MediaSegment>> {
        let content = if url.starts_with("http") {
            if !url.split("?").next().unwrap().ends_with(".m3u8"){
                self.scrape_website()?;
            }
            self.downloader.get_bytes(&url)?
        } else {
            std::fs::read_to_string(&url)?.as_bytes().to_vec()
        };
        
        if !output.is_empty() {
            self.args.output = Some(output);
        }
    
        self.parse_playlist(content)
    }

    pub fn parse_playlist(&mut self, content:Vec<u8>) -> Result<Vec<m3u8_rs::MediaSegment>> {
        match m3u8_rs::parse_playlist_res(&content)
            .map_err(|_| anyhow!("Couldn't parse {} playlist.", self.args.input))?
        {
            m3u8_rs::Playlist::MasterPlaylist(master) => {
                self.args.input = if self.args.alternative {
                    self.get_url(&parse::alternative(&master, self.args.raw_prompts)?)?
                } else {
                    self.get_url(&parse::master(
                        &master,
                        &self.args.quality,
                        self.args.raw_prompts,
                    )?)?
                };

                self.progress.current("stream");
                self.progress.stream = StreamData::new(&self.args.input, &self.tempfile());
                self.progress
                    .json_file(&replace_ext(&self.progress.stream.file, "json"));

                if !self.args.alternative && !self.args.skip {
                    self.download_alternative(&master)?;
                }

                let playlist = self.downloader.get_bytes(&self.args.input).unwrap();
                match m3u8_rs::parse_playlist_res(&playlist)
                    .map_err(|_| anyhow!("Couldn't parse {} playlist.", self.args.input))?
                {
                    m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                        println!(
                            "{} {} stream.",
                            "Downloading".colorize("bold green"),
                            if self.args.alternative {
                                "alternative"
                            } else {
                                "video"
                            }
                        );
                        return Ok(meadia.segments);
                    }
                    _ => bail!("Media playlist not found."),
                }
            }
            m3u8_rs::Playlist::MediaPlaylist(meadia) => {
                println!("{} video stream.", "Downloading".colorize("bold green"));
                self.progress.current("stream");
                self.progress.stream = StreamData::new(&self.args.input, &self.tempfile());
                self.progress
                    .json_file(&replace_ext(&self.progress.stream.file, "json"));
                return Ok(meadia.segments);
            }
        }
    }

    pub fn download(
        &self,
        segments: &Vec<m3u8_rs::MediaSegment>,
        mut tempfile: String,
    ) -> Result<()> {
        // Check to ensure baseurl is required or not.
        self.get_url(&segments[0].uri)?;

        if let Some(output) = &self.args.output {
            if output.ends_with(".ts") {
                tempfile = output.clone();
            }
            println!(
                "File will be saved at {}",
                tempfile.clone().colorize("cyan")
            );
        } else {
            println!(
                "Temporary file will be saved at {}",
                tempfile.clone().colorize("cyan")
            );
        }

        let total = segments.len();
        let merger = if self.args.resume {
            Arc::new(Mutex::new(BinarySequence::try_from_json(
                total,
                tempfile.clone(),
                self.progress.json_file.clone(),
            )?))
        } else {
            Arc::new(Mutex::new(BinarySequence::new(
                total,
                tempfile.clone(),
                self.progress.clone(),
            )?))
        };
        merger.lock().unwrap().update()?;

        let pb = Arc::new(Mutex::new(kdam::tqdm!(
            total = total,
            unit = "ts".to_owned(),
            dynamic_ncols = true
        )));
        let client = Arc::new(self.downloader.clone());
        let pool = threadpool::ThreadPool::new(self.args.threads as usize);
        PROG.lock().unwrap().total = total;
        PROG.lock().unwrap().downloaded = 0;

        for (i, segment) in segments.iter().enumerate() {
            if self.args.resume {
                let pos = merger.lock().unwrap().position();

                if pos != 0 {
                    if pos >= i + 1 {
                        continue;
                    }
                }
            }

            if let Some(m3u8_key) = &segment.key {
                if m3u8_key.method == "SAMPLE-AES" {
                    bail!("SAMPLE-AES encrypted playlists are not supported.")
                }
            }

            let key_url = match &segment.key {
                Some(m3u8_rs::Key {
                    uri: Some(link), ..
                }) => Some(self.get_url(&link)?),
                _ => None,
            };

            let segment = segment.clone();
            let pb = pb.clone();
            let merger = merger.clone();
            let client = client.clone();
            let segment_url = self.get_url(&segment.uri)?;
            let total_retries = self.args.retry_count.clone();

            let merger_c = merger.clone();
            let merger_cm = merger_c.lock().unwrap();

            pb.lock().unwrap().set_description(format!(
                "{} / {}",
                format_bytes(merger_cm.stored()).2,
                format_bytes(merger_cm.estimate()).2
            ).as_str());
            pb.lock().unwrap().update_to(merger_cm.position());
            PROG.lock().unwrap().downloaded = merger_cm.position();

            let obj = self.speed.clone();

            pool.execute(move || {
                let mut retries = 0;

                let mut data = loop {
                    let resp = match segment.byte_range {
                        Some(m3u8_rs::ByteRange {
                            length: start,
                            offset: Some(end),
                        }) => client.get_bytes_range(&segment_url, start, start + end - 1),
                        _ => client.get_bytes(&segment_url),
                    };

                    if resp.is_ok() {
                        break resp.unwrap();
                    } else {
                        if total_retries > retries {
                            pb.lock().unwrap().write(format!(
                                "{} to download segment at index {}.",
                                "RETRYING".colorize("bold yellow"),
                                i
                            ).as_str());
                            retries += 1;
                            continue;
                        } else {
                            pb.lock().unwrap().write(format!(
                                "{}: Reached maximum number of retries for segment at index {}.",
                                "Error".colorize("bold red"),
                                i
                            ).as_str());
                            std::process::exit(1);
                        }
                    }
                };

                // Decrypt
                retries = 0;

                if let Some(eku) = key_url {
                    data = loop {
                        let resp = client.get_bytes(&eku);

                        if resp.is_ok() {
                            let decrypted_data = crate::vsd::decrypt::HlsDecrypt::from_key(
                                segment.key.unwrap(),
                                resp.unwrap(),
                            )
                            .decrypt(&data);

                            break decrypted_data.unwrap_or_else(|e| {
                                pb.lock().unwrap().write(format!(
                                    "{}: {}",
                                    "Error".colorize("bold red"),
                                    e
                                ).as_str());
                                std::process::exit(1);
                            });
                        } else {
                            if total_retries > retries {
                                pb.lock().unwrap().write(format!(
                                    "{} to download decryption key.",
                                    "RETRYING".colorize("bold yellow"),
                                ).as_str());
                                retries += 1;
                                continue;
                            } else {
                                pb.lock().unwrap().write(format!(
                                "{}: Reached maximum number of retries to download decryption key.",
                                "Error".colorize("bold red"),
                            ).as_str());
                                std::process::exit(1);
                            }
                        }
                    };
                }

                let mut merger = merger.lock().unwrap();
                merger.write(i, &data).unwrap();
                merger.flush().unwrap();

                let mut pb = pb.lock().unwrap();
                pb.set_description(format!(
                    "{} / {}",
                    format_bytes(merger.stored()).2,
                    format_bytes(merger.estimate()).2
                ).as_str());
                pb.update(1);
                PROG.lock().unwrap().downloaded += 1;

                obj.lock().unwrap().downloaded += 1;
            });
        }

        pool.join();
        eprintln!();
        merger.lock().unwrap().flush().unwrap();

        if merger.lock().unwrap().buffered() {
            println!(
                "File {} downloaded successfully.",
                tempfile.colorize("bold green")
            );
        } else {
            bail!(
                "File {} not downloaded successfully.",
                tempfile.colorize("bold red")
            );
        }

        Ok(())
    }

    pub fn transmux_trancode(&mut self) -> Result<()> {
        if let Some(output) = &self.args.output {
            if output.ends_with(".ts") {
                return Ok(());
            }

            let mut args = vec!["-i", &self.progress.stream.file];

            if let Some(audio) = &self.progress.audio {
                args.push("-i");
                args.push(&audio.file);
            }

            if let Some(subtitle) = &self.progress.subtitle {
                args.push("-i");
                args.push(&subtitle.file);
            }

            if std::path::Path::new(output).exists() {
                std::fs::remove_file(output)?;
            }

            args.push("-c");
            args.push("copy");
            args.push(output);

            println!(
                "Executing {} {}",
                "ffmpeg".colorize("cyan"),
                args.join(" ").colorize("cyan")
            );

            let code = std::process::Command::new("ffmpeg")
                .args(args)
                .stderr(std::process::Stdio::null())
                .spawn()?
                .wait()?;

            if !code.success() {
                bail!("FFMPEG exited with code {}.", code.code().unwrap_or(1))
            }

            if let Some(audio) = &self.progress.audio {
                std::fs::remove_file(&audio.file)?;
            }

            if let Some(subtitle) = &self.progress.subtitle {
                std::fs::remove_file(&subtitle.file)?;
            }

            std::fs::remove_file(&self.progress.stream.file)?;
        }

        if std::path::Path::new(&self.progress.json_file).exists() {
            std::fs::remove_file(&self.progress.json_file)?;
        }
        Ok(())
    }
}
