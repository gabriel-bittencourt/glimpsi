use log::LevelFilter;
use chrono::Local;
use fern;
use tauri::{
    AppHandle, Runtime
};

use crate::config;
use crate::config::types::AppFile;


pub fn setup_logger<R: Runtime>(app: &AppHandle<R>) -> Result<(), fern::InitError> {
    let path = config::file::get_file_path(app, AppFile::Log);
    let file = fern::log_file(path)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
                record.level(),                           // Level
                record.target(),                          // Module
                message                                   // Log message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout()) // remove this for logging to file only
        .chain(file)
        .apply()?;

    Ok(())
}
