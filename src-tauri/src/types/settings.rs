use serde::{Deserialize, Serialize};
use crate::types::shortcut::SettingsShortcut;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub global_shortcut: SettingsShortcut
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            global_shortcut: SettingsShortcut::default()
        }
    }
}
