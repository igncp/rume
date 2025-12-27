use std::collections::HashSet;

use tracing::info;

use crate::rume::{
    key_table::{RumeKeyModifier, RumeKeyTable},
    session::{RumeSession, RumeSessionId},
};

pub struct Engine {
    pub(super) session_id: RumeSessionId,
}

impl Engine {
    pub fn process_key(
        &self,
        session: &RumeSession,
        key: RumeKeyTable,
        modifiers: &HashSet<RumeKeyModifier>,
    ) -> Result<bool, String> {
        let modifiers_str = modifiers
            .iter()
            .map(|m| format!("{m}"))
            .collect::<Vec<String>>()
            .join(", ");
        let commited_text = &session.commited_text;
        let session_id = self.session_id;

        info!("Key down event received: session_id='{session_id}' key='{key}' with modifiers='{modifiers_str}', commited_text='{commited_text}'");

        Ok(false)
    }
}
