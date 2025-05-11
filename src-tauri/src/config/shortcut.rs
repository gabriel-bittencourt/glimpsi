/// This module is responsible for mapping the internal shortcut representation to the global shortcut representation (used by the tauri plugin).


use std::str::FromStr;
use tauri_plugin_global_shortcut::{
    Code as GlobalKey,
    Modifiers as GlobalModifier,
    Shortcut as GlobalShortcut
};

use crate::config::types::Shortcut as InternalShortcut;
use shared::types::modifier::Modifier as InternalModifier;


struct Modifiers(Vec<InternalModifier>);
impl From<Modifiers> for GlobalModifier {
    fn from(modifiers: Modifiers) -> Self {
        let mut global_modifier = GlobalModifier::empty();
        if modifiers.0.len() == 0 {
            return global_modifier;
        }

        for modif in modifiers.0 {
            match modif {
                InternalModifier::Ctrl | InternalModifier::Control => global_modifier.insert(GlobalModifier::CONTROL),
                InternalModifier::Shift | InternalModifier::ShiftArrow => global_modifier.insert(GlobalModifier::SHIFT),
                InternalModifier::Alt => global_modifier.insert(GlobalModifier::ALT),
                InternalModifier::AltGr => global_modifier.insert(GlobalModifier::ALT_GRAPH),
                InternalModifier::Meta | InternalModifier::Windows | InternalModifier::Command => global_modifier.insert(GlobalModifier::META),
                _ => (),
            }
        }
    
        global_modifier
    }
}


struct Key(String);
impl From<Key> for GlobalKey {
    fn from(key: Key) -> Self {
        let mut key_string: String = key.0.to_owned();

        let mut chars = key.0.chars();

        if let Some(c) = chars.next() {
            if chars.next().is_none() && c.is_ascii_alphabetic() {
                key_string = "Key".to_owned() + key.0.as_ref();
            }
        }

        GlobalKey::from_str(&key_string).unwrap() // TODO: remove unwrap
    }
}


pub struct Shortcut(pub InternalShortcut);
impl From<Shortcut> for GlobalShortcut {
    fn from(shortcut: Shortcut) -> Self {
        GlobalShortcut::new(
            Some(GlobalModifier::from(Modifiers(shortcut.0.modifiers))),
            GlobalKey::from(Key(shortcut.0.key))
        )
    }
}

