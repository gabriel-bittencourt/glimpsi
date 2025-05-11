use tauri::{Result, Runtime, WebviewWindow};


#[tauri::command]
pub fn show<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    match window.show() {
        Ok(_) => window.set_focus(),
        Err(e) => {
            eprintln!("Error showing window: {}", e);
            Err(e)
        }
    }
}

#[tauri::command]
pub fn hide<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    window.hide()
}

#[tauri::command]
pub fn restart<R: Runtime>(window: &WebviewWindow<R>) -> Result<()> {
    window.eval("window.location.reload()")
}