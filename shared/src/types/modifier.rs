use serde::{Deserialize, Serialize};
use std::fmt;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Modifier {
    Ctrl,       // Ctrl
    Control,    // ^
    Shift,      // Shift
    ShiftArrow, // ⇧
    Alt,        // Alt
    AltGr,      // AltGr
    Option,     // ⌥
    Meta,       // Meta
    Windows,    // ⊞
    Command,    // ⌘
    Function,   // Fn
    None
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Modifier::Ctrl => write!(f, "Ctrl"),
            Modifier::Control => write!(f, "^"),
            Modifier::Shift => write!(f, "Shift"),
            Modifier::ShiftArrow => write!(f, "⇧"),
            Modifier::Alt => write!(f, "Alt"),
            Modifier::AltGr => write!(f, "AltGr"),
            Modifier::Option => write!(f, "⌥"),
            Modifier::Meta => write!(f, "Meta"),
            Modifier::Windows => write!(f, "⊞"),
            Modifier::Command => write!(f, "⌘"),
            Modifier::Function => write!(f, "Fn"),
            Modifier::None => write!(f, ""),
        }
    }
}
