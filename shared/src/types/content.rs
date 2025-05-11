use serde::{Deserialize, Serialize};
use crate::types::{
    modifier::Modifier,
    shortcut::Shortcut
};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    pub title: String,
    pub shortcuts: Vec<Shortcut>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Column {
    pub applications: (Application, Option<Application>) // A column can have one or two applications
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Content {
    pub columns: (Column, Option<Column>) // A content can have one or two columns
}

impl Default for Content {
    fn default() -> Self {

        let modifiers = vec![Modifier::Ctrl, Modifier::Shift];
        let mut description: String;

        let mut shortcuts: Vec<Shortcut> = Vec::new();

        for i in 0..4 {
            description = format!("Shortcut {}", i + 1);
            shortcuts.push(Shortcut {
                description,
                modifiers: modifiers.clone(),
                key: "K".to_string()
            });
        }

        Content { columns: (
            Column {
                applications: (
                    Application {
                        title: "Application 1".to_string(),
                        shortcuts: shortcuts.clone()
                    },
                    Some(Application {
                        title: "Application 2".to_string(),
                        shortcuts: shortcuts.clone()
                    })
                ),
            },
            None
        )}

    }
}
