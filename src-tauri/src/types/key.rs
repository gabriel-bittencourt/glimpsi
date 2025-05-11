use std::str::FromStr;
use tauri_plugin_global_shortcut::Code as GlobalKey;


pub struct Key(pub String);
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
