use serde::{Deserialize, Serialize};
use shared::types::{
    content::{
        Application as InternalApplication,
        Column as InternalColumn,
        Content as InternalContent
    },
    modifier::Modifier as InternalModifier,
    shortcut::Shortcut as InternalShortcut
};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileContent {
    applications: Vec<InternalApplication>
}

impl Default for FileContent {
    fn default() -> Self {

        let modifiers = vec![InternalModifier::Ctrl, InternalModifier::Shift];
        let mut description: String;

        let mut shortcuts: Vec<InternalShortcut> = Vec::new();
        let mut applications: Vec<InternalApplication> = Vec::new();

        for i in 0..4 {
            description = format!("Shortcut {}", i + 1);
            shortcuts.push(InternalShortcut {
                description,
                modifiers: modifiers.clone(),
                key: "K".to_string()
            });
        }
        
        for i in 0..3 {
            applications.push(InternalApplication {
                title: format!("Application {}", i + 1),
                shortcuts: shortcuts.clone()
            });
        }

        FileContent { applications }
    }
}


impl From<FileContent> for InternalContent {
    fn from(content: FileContent) -> Self {
        let mut apps = content.applications.into_iter();

        let first_column = InternalColumn {
            applications: (
                apps.next().expect("Expected at least one application"),
                apps.next()
            )
        };

        let second_column = match (apps.next(), apps.next()) {
            (Some(app1), app2) => Some(InternalColumn { applications: (app1, app2) }),
            _ => None
        };

        InternalContent {
            columns: (first_column, second_column)
        }
    }
}
