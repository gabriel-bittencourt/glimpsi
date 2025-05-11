use tauri::{AppHandle, Result, Runtime};


#[tauri::command]
pub fn quit<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    app.exit(0);

    Ok(())
}
