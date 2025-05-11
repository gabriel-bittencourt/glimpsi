use tauri_plugin_global_shortcut::Modifiers as GlobalModifier;

// use crate::config::types::Shortcut as InternalShortcut;
use shared::types::modifier::Modifier as InternalModifier;


pub struct Modifiers(pub Vec<InternalModifier>);
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
