use std::fmt;
use serde::{Deserialize, Serialize};
use shared::types::{content::Application, modifier::Modifier, shortcut::Shortcut as ContentShortcut};


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
pub struct Content {
    applications: Vec<Application>
}

impl Default for Content {
    fn default() -> Self {

        let modifiers = vec![Modifier::Ctrl, Modifier::Shift];
        let mut description: String;

        let mut shortcuts: Vec<ContentShortcut> = Vec::new();
        let mut applications: Vec<Application> = Vec::new();

        for i in 0..4 {
            description = format!("Shortcut {}", i + 1);
            shortcuts.push(ContentShortcut {
                description,
                modifiers: modifiers.clone(),
                key: "K".to_string()
            });
        }
        
        for i in 0..3 {
            applications.push(Application {
                title: format!("Application {}", i + 1),
                shortcuts: shortcuts.clone()
            });
        }

        Content { applications }
    }
}
