use tauri::{AppHandle, Result, Runtime};


pub fn quit<R: Runtime>(app: &AppHandle<R>) -> Result<()> {
    app.exit(0);

    Ok(())
}
