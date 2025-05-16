use log::error;
use tauri::{AppHandle, Manager, Result, Runtime, WebviewWindow};
use tauri_plugin_opener::OpenerExt;

use crate::types::{
    app_file::AppFile,
    state::AppState
};
use crate::config::file;
use crate::events::window;

pub fn quit<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    app.exit(0);

    Ok(())
}

pub fn reload_content<R: Runtime>(app: &AppHandle<R>, window: &WebviewWindow<R>) -> Result<()> {
    let state = app.state::<AppState>();

    match state.content.lock() {
        Ok(mut data_guard) => {
            *data_guard = file::load_data(app, AppFile::Content);
        }
        _ => {}
    }

    window::reload(window)
}

pub fn open_config_folder<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    let path = match file::get_config_path(app) {
        Ok(path) => path,
        Err(e) => {
            error!("Error getting config path: {}", e);
            return Err(e);
        }
    };

    match app.opener().open_path(path.to_string_lossy(), None::<&str>) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Error opening config folder: {}", e);
            Ok(())
        }
    }
}
