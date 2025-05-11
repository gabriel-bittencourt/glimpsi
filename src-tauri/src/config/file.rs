use std::error::Error;
use std::fs;
use std::path::PathBuf;
use log::warn;
use serde::{Serialize, de::DeserializeOwned};
use tauri::{
    AppHandle, Manager, Runtime
};


use crate::config::types::AppFile;


fn create_path_if_not_exists(path: &PathBuf){
    if !path.exists() {
        std::fs::create_dir_all(path)
            .expect(&format!("Failed to create directory: {}", path.display()));
    }
}

pub fn get_config_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, Box<dyn Error>> {
    let path = app.path().app_config_dir()?;
    create_path_if_not_exists(&path);

    Ok(path)
}

pub fn get_file_path<R: Runtime>(app: &AppHandle<R>, app_file: AppFile) -> PathBuf {
    let path = get_config_path(app).unwrap(); // TODO: remove unwrap

    match app_file {
        AppFile::Settings => path.join("settings.toml"),
        AppFile::Content => path.join("content.toml"),
        AppFile::Log => path.join("log.txt")
    }
}

fn write_data<T: Serialize>(path: &PathBuf, data: &T) -> Result<(), Box<dyn Error>> {
    let contents = toml::to_string_pretty(data)?; 
    fs::write(&path, contents)?;

    Ok(())
}

fn read_data<T: DeserializeOwned>(path: &PathBuf) -> Result<T, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let data = toml::de::from_str::<T>(&contents)?;

    Ok(data)
}

/// Tries to load data from a file into struct of type T. If it fails, return default 
fn read_data_or_default<T>(path: &PathBuf) -> T
where 
    T: DeserializeOwned + Default
{
    match read_data(path) {
        Ok(data) => data,
        Err(e) => {
            warn!("Failed to read file. Error: {}. Initializing with default values.", e);
            T::default()
        }
    }
}

/// Tries to load data from a file into struct of type T.
///
/// If the file does not exist, creates a new file with default values.
// TODO: remove app_file param and infer file path (and display name) from T
pub fn load_data<R, T>(app: &AppHandle<R>, app_file: AppFile) -> T
where
    R: Runtime,
    T: Serialize + DeserializeOwned + Default
{
    let data: T;
    let path = get_file_path(app, app_file.clone());

    if path.exists() {
        return read_data_or_default(&path);
    }

    data = T::default();
    match write_data(&path, &data) {
        Ok(_) => {},
        Err(e) => {
            warn!("Failed to create file. Error: {}", e);
            return data;
        }
    };
    warn!(
        "{} file missing. Initializing with default values and creating file at: {}",
        app_file,
        path.display()
    );
    
    data
}
