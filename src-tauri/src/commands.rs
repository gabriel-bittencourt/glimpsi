/// Commands called from the frontend

use log::info;
use tauri::State;

use crate::types::content::FileContent;
use shared::types::content::Content;


#[tauri::command]
pub fn get_content(state: State<FileContent>) -> Content {
    info!("get_content called");

    Content::from(state.inner().to_owned())
}
