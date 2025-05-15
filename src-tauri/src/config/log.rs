use log::LevelFilter;
use chrono::Local;
use fern;
use tauri::{
    AppHandle, Runtime
};

use crate::config;
use crate::types::app_file::AppFile;


pub fn setup_logger<R: Runtime>(app: &AppHandle<R>) -> Result<(), fern::InitError> {
    let path = config::file::get_file_path(app, AppFile::Log);
    let file = fern::log_file(path)?;

    fern::Dispatch::new()
        .format(|out, message, _| {
            out.finish(format_args!(
                "[{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"), // Timestamp
                message                                   // Log message
            ))
        })
        .level(LevelFilter::Info)
        .chain(std::io::stdout()) // remove this for logging to file only
        .chain(file)
        .apply()?;

    // TODO: implement some kind of log rotation

    Ok(())
}
