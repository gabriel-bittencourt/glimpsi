use dioxus::prelude::*;

use shared::types::{
    modifier::Modifier,
    shortcut::Shortcut,
    content::{
        Application,
        Column,
        Content
    }
};


#[component]
pub fn Key(kbd_key: String) -> Element {
    rsx! {
        span {
            class:"kbd-key",
            "{kbd_key}"
        }
    }
}

#[component]
pub fn KeyModifier(kbd_modifier: Modifier) -> Element {
    rsx! {
        span {
            class:"kbd-key kbd-modifier",
            "{kbd_modifier}"
        }
    }
}

#[component]
fn ShortcutDescription(description: String) -> Element {
    rsx! {
        span {
            class: "shortcut-description",
            "{description}"    
        }
    }
}

#[component]
fn ShortcutModifiers(modifiers: Vec<Modifier>) -> Element {
    rsx! {
        span {
            class: "shortcut-keys",
            for modifier in modifiers {
                KeyModifier { kbd_modifier: modifier }
            }
        }    
    }
}

#[component]
pub fn ShortcutItem(shortcut: Shortcut) -> Element {
    rsx! {
        li {
            class: "shortcut-item",
            ShortcutDescription { description: shortcut.description },
            span {
                class: "shortcut-keys",
                ShortcutModifiers { modifiers: shortcut.modifiers },
                Key { kbd_key: shortcut.key }
            }
        }
    }
}

#[component]
fn ApplicationTitle(title: String) -> Element {
    rsx! {
        h3 {
            class: "application-title",
            "{title}"
        }
    }
}

#[component]
fn ApplicationContent(content: Vec<Shortcut>) -> Element {
    rsx! {
        ul {
            class: "application-content",
            for item in content {
                ShortcutItem { shortcut: item }    
            }
        }
    }
}

#[component]
pub fn ApplicationComponent(application: Option<Application>) -> Element {
    match application {
        Some(application) => {
            rsx! {
                div {
                    class: "application",
                    ApplicationTitle { title: application.title },
                    ApplicationContent { content: application.shortcuts }
                }
            }
        },
        None => rsx!()
    }

}

#[component]
pub fn ColumnComponent(column: Option<Column>) -> Element {
    match column {
        Some(column) => {
            rsx! {
                div {
                    class: "column",
                    ApplicationComponent { application: Some(column.applications.0) },
                    ApplicationComponent { application: column.applications.1 }
                }
            }
        },
        None => rsx!()
    }
}

#[component]
pub fn ContentComponent(content: Content) -> Element {
    rsx! {
        ColumnComponent { column: Some(content.columns.0) },
        ColumnComponent { column: content.columns.1 }
    }
}
