use tauri::State;

use shared::types::content::Content;


#[tauri::command]
pub fn get_content(content: State<Content>) -> Content {
    content.inner().clone()
}
