use std::collections::{HashMap, HashSet};

use tracing::{debug, info};

use crate::rume::{
    engine::Engine,
    key_table::{RumeKeyModifier, RumeKeyTable},
    logger::setup_logs,
    session::{RumeSession, RumeSessionId},
};

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

pub struct Rume {
    pub rume_config: Option<RumeNewConfig>,
    initialized: bool,
    sessions: HashMap<RumeSessionId, RumeSession>,
    last_session_id: RumeSessionId,
}

impl Rume {
    pub fn new(opt: Option<RumeNewConfig>) -> Self {
        Self {
            rume_config: opt,
            initialized: false,
            sessions: HashMap::new(),
            last_session_id: 0,
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

    // true: event handled, false: not handled
    pub fn process_key(
        &mut self,
        session_id: RumeSessionId,
        key: RumeKeyTable,
        modifiers: HashSet<RumeKeyModifier>,
    ) -> Result<bool, String> {
        let Some(session) = self.sessions.get_mut(&session_id) else {
            let err_msg = format!("Session id={} not found", session_id);
            info!("{}", err_msg);
            return Err(err_msg);
        };

        session.commited_text.push_str(&key.to_string());

        session.engine.process_key(session, key, &modifiers)
    }

    pub fn create_session(&mut self) -> RumeSessionId {
        self.last_session_id += 1;
        let engine = Engine {
            session_id: self.last_session_id,
        };
        let session = RumeSession {
            id: self.last_session_id,
            engine,
            commited_text: String::new(),
        };
        self.sessions.insert(session.id, session);
        info!("Created session with id={}", self.last_session_id);
        self.last_session_id
    }

    pub fn delete_session(&mut self, session_id: RumeSessionId) {
        info!("Deleting session with id={}", session_id);
        self.sessions.remove(&session_id);
    }
}
