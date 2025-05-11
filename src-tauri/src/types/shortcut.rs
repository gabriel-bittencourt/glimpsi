use serde::{Deserialize, Serialize};
use tauri_plugin_global_shortcut::{
    Code as GlobalKey,
    Modifiers as GlobalModifier,
    Shortcut as GlobalShortcut
};

use shared::types::modifier::Modifier as InternalModifier;
use crate::types::{
    key::Key,
    modifier::Modifiers
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SettingsShortcut {
    pub modifiers: Vec<InternalModifier>,
    pub key: String
}

impl From<SettingsShortcut> for GlobalShortcut {
    fn from(shortcut: SettingsShortcut) -> Self {
        GlobalShortcut::new(
            Some(GlobalModifier::from(Modifiers(shortcut.modifiers))),
            GlobalKey::from(Key(shortcut.key))
        )
    }
}

impl Default for SettingsShortcut {
    fn default() -> Self {
        SettingsShortcut {
            modifiers: vec![InternalModifier::Ctrl, InternalModifier::Shift],
            key: "Q".to_string()
        }
    }
}
