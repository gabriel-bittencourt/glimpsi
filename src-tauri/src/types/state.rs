use std::sync::Mutex;
use crate::types::content::FileContent;


pub struct AppState {
    pub content: Mutex<FileContent>
}
