use std::collections::HashSet;

use tracing::info;

use crate::rume::{
    key_table::{RumeKeyModifier, RumeKeyTable},
    session::{RumeSession, RumeSessionId},
};

pub struct Engine {
    pub(super) session_id: RumeSessionId,
}

// These could be handled in the future to navigate the preedit
/// cbindgen:ignore
const SKIP_KEYS: &[RumeKeyTable] = &[
    RumeKeyTable::ArrowDown,
    RumeKeyTable::ArrowLeft,
    RumeKeyTable::ArrowRight,
    RumeKeyTable::ArrowUp,
];

pub enum EngineProcessKeyResult {
    Handled {
        preedit_text: String,
        commited_text: String,
    },
    NotHandled,
}

impl Engine {
    pub fn process_key(
        &self,
        session: &RumeSession,
        key: RumeKeyTable,
        modifiers: &HashSet<RumeKeyModifier>,
    ) -> Result<EngineProcessKeyResult, String> {
        if SKIP_KEYS.contains(&key)
            || key == RumeKeyTable::Enter && session.context.preedit_text.is_empty()
        {
            return Ok(EngineProcessKeyResult::NotHandled);
        }

        let modifiers_str = modifiers
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join(", ");
        let session_id = self.session_id;
        let mut preedit_text = session.context.preedit_text.clone();
        let mut commited_text = String::new();
        match key {
            RumeKeyTable::Backspace => {
                preedit_text.pop();
            }
            RumeKeyTable::Escape => {
                preedit_text.clear();
            }
            RumeKeyTable::Enter => {
                commited_text.push_str(&preedit_text);
                preedit_text.clear();
            }
            _ => {
                preedit_text.push_str(&format!("{key}"));
            }
        };

        info!("Key down event received: session_id='{session_id}' key='{key}' with modifiers='{modifiers_str}', preedit_text='{preedit_text}', commited_text='{commited_text}'");

        Ok(EngineProcessKeyResult::Handled {
            preedit_text,
            commited_text,
        })
    }
}
