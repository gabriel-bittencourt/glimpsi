use tauri::{AppHandle, Manager, Result, Runtime, WebviewWindow};

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
