use std::collections::{HashMap, HashSet};

use tracing::{debug, info};

use crate::rume::{
    engine::{Engine, EngineProcessKeyResult},
    key_table::{RumeKeyModifier, RumeKeyTable},
    logger::setup_logs,
    session::{RumeContext, RumeMenu, RumeSession, RumeSessionId},
};

pub mod bin_parser;
pub mod config_handler;
pub mod engine;
pub mod key_table;
pub mod logger;
pub mod session;
pub mod version;

#[derive(Clone)]
pub struct RumeNewConfig {
    pub app_name: String,
    pub min_log_level: Option<u32>,
    pub log_dir: Option<String>,
    pub stdout_log: bool,
}

impl Default for RumeNewConfig {
    fn default() -> Self {
        Self {
            app_name: "rume_unknown".to_string(),
            min_log_level: None,
            log_dir: None,
            stdout_log: true,
        }
    }
}

pub enum ProcessKeyResult {
    Handled,
    Enabled,
    Disabled,
    NotHandled,
}

pub struct Rume {
    pub rume_config: Option<RumeNewConfig>,
    initialized: bool,
    sessions: HashMap<RumeSessionId, RumeSession>,
    last_session_id: RumeSessionId,
    is_enabled: bool,
}

impl Rume {
    pub fn new(opt: Option<RumeNewConfig>) -> Self {
        Self {
            rume_config: opt,
            initialized: false,
            sessions: HashMap::new(),
            last_session_id: 0,
            is_enabled: true,
        }
    }

    pub fn init(&mut self) -> Result<(), String> {
        if self.initialized {
            debug!("Rume already initialized");
            return Ok(());
        }
        self.initialized = true;

        let rume_config = self.rume_config.clone().unwrap_or_default();
        setup_logs(
            &rume_config.app_name,
            rume_config.log_dir,
            rume_config.stdout_log,
        );

        info!("Rume initializing...");
        info!("Rume initialized");

        Ok(())
    }

    pub fn process_key(
        &mut self,
        session_id: RumeSessionId,
        key: RumeKeyTable,
        modifiers: HashSet<RumeKeyModifier>,
    ) -> Result<ProcessKeyResult, String> {
        if key == RumeKeyTable::Equal && modifiers.contains(&RumeKeyModifier::Control) {
            self.is_enabled = !self.is_enabled;
            info!("Rume enabled set to {}", self.is_enabled);

            return Ok(if self.is_enabled {
                ProcessKeyResult::Enabled
            } else {
                ProcessKeyResult::Disabled
            });
        }

        if !self.is_enabled {
            return Ok(ProcessKeyResult::NotHandled);
        }

        let Some(session) = self.sessions.get_mut(&session_id) else {
            let err_msg = format!("Session id={} not found", session_id);
            info!("{}", err_msg);
            return Err(err_msg);
        };

        let Ok(handling_result) = session.engine.process_key(session, key, &modifiers) else {
            return Ok(ProcessKeyResult::NotHandled);
        };

        match handling_result {
            EngineProcessKeyResult::Handled {
                preedit_text,
                commited_text,
            } => {
                session.context.preedit_text = preedit_text;
                // Hide candidates when there is no preedit text
                session.context.menu.num_candidates = if session.context.preedit_text.is_empty() {
                    0
                } else {
                    5
                };
                session.commited_text = commited_text;
                Ok(ProcessKeyResult::Handled)
            }
            EngineProcessKeyResult::NotHandled => Ok(ProcessKeyResult::NotHandled),
        }
    }

    pub fn create_session(&mut self) -> RumeSessionId {
        self.last_session_id += 1;
        let engine = Engine {
            session_id: self.last_session_id,
        };
        // Start with zero candidates until there is preedit text
        let menu = RumeMenu { num_candidates: 0 };
        let preedit_text = String::new();
        let context = RumeContext { menu, preedit_text };
        let session = RumeSession {
            id: self.last_session_id,
            engine,
            commited_text: String::new(),
            context,
        };
        self.sessions.insert(session.id, session);
        info!("Created session with id={}", self.last_session_id);
        self.last_session_id
    }

    pub fn delete_session(&mut self, session_id: RumeSessionId) {
        info!("Deleting session with id={}", session_id);
        self.sessions.remove(&session_id);
    }

    pub fn get_commit(&self, session_id: RumeSessionId) -> Option<String> {
        let Some(session) = self.sessions.get(&session_id) else {
            let err_msg = format!("Session id={} not found", session_id);
            info!("{}", err_msg);
            return None;
        };

        if session.commited_text.is_empty() {
            return None;
        }

        let commit_text = session.commited_text.clone();

        Some(commit_text)
    }

    pub fn get_session(&self, session_id: RumeSessionId) -> Option<&RumeSession> {
        self.sessions.get(&session_id)
    }
}
