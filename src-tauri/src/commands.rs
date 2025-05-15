/// Commands called from the frontend

use crate::types::state::AppState;
use shared::types::content::Content;


#[tauri::command]
pub fn get_content(state: tauri::State<AppState>) -> Content {
    let content = state.content.lock().unwrap(); // TODO: remove unwrap

    Content::from(content.to_owned())
}
