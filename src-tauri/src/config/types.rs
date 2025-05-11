use std::fmt;
use serde::{Deserialize, Serialize};
use shared::types::modifier::Modifier;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shortcut {
    pub modifiers: Vec<Modifier>,
    pub key: String
}

impl Default for Shortcut {
    fn default() -> Self {
        Shortcut {
            modifiers: vec![Modifier::Ctrl, Modifier::Shift],
            key: "Q".to_string()
        } 
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub global_shortcut: Shortcut
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            global_shortcut: Shortcut::default()
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AppFile {
    Settings,
    Content,
    Log
}

impl fmt::Display for AppFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppFile::Settings => write!(f, "Settings"),
            AppFile::Content => write!(f, "Content"),
            AppFile::Log => write!(f, "Log")            
        }
    }
}


