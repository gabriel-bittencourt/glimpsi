use serde::{Deserialize, Serialize};
use crate::types::modifier::Modifier;


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Shortcut {
    pub description: String,
    pub modifiers: Vec<Modifier>,
    pub key: String
}
