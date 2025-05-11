/// Defines the file types used in the application.

use std::fmt;
use serde::{Deserialize, Serialize};


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
