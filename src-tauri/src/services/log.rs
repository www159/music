const SERIVCE_DIR: &str = "log";

use std::{fs};
use log::LevelFilter;
use tauri::api::path::home_dir;
use chrono::Local;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use crate::applications::APP_DIR;


pub fn init_log() -> anyhow::Result<()> {
    let log_dir = home_dir()
        .ok_or(anyhow::anyhow!("failed to get home dir"))?
        .join(APP_DIR)
        .join(SERIVCE_DIR);

    if !log_dir.exists() {
        fs::create_dir_all(&log_dir).unwrap();
    }

    #[cfg(feature = "music-dev")]
    let log_partten = "{d(%Y-%m-%d %H:%M:%S)} {l} - {M}: {m}{n}";

    #[cfg(not(feature = "music-dev"))]
    let log_partten = "{d(%Y-%m-%d %H:%M:%S)} {l} - {m}{n}";
    
    let encoder = Box::new(PatternEncoder::new(log_partten));
    
    let stdout = ConsoleAppender::builder()
    .encoder(encoder.clone())
    .build();

    let local_time = Local::now().format("%Y-%m-%d-%H:%M").to_string();
    
    let log_file = format!("{}.log", local_time);
    let log_file = log_dir.join(log_file);
    let log_file = FileAppender::builder()
        .encoder(encoder)
        .build(log_file)?;

    #[cfg(feature = "music-dev")]
    let level = LevelFilter::Debug;
    #[cfg(not(feature = "music-dev"))]
    let level = LevelFilter::Info;

    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(log_file)))
        .logger(
            Logger::builder()
                .appenders(["file", "stdout"])
                .additive(false)
                .build("app", level),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))?;

    log4rs::init_config(log_config)?;
    Ok(())
}