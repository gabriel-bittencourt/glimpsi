/// Handles window events

use tauri::{Result, Runtime, WebviewWindow};


pub fn show<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    match window.show() {
        Ok(_) => window.set_focus(),
        Err(e) => {
            eprintln!("Error showing window: {}", e);
            Err(e)
        }
    }
}

pub fn hide<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    window.hide()
}

pub fn reload<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    window.eval("window.location.reload()")
}
