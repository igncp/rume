use crate::rume::engine::Engine;

pub type RumeSessionId = usize;

#[derive(Clone)]
pub struct RumeMenu {
    pub num_candidates: usize,
}

#[derive(Clone)]
pub struct RumeContext {
    pub menu: RumeMenu,
    pub preedit_text: String,
}

pub struct RumeSession {
    pub(crate) id: RumeSessionId,
    pub(crate) engine: Engine,
    pub(crate) commited_text: String,
    pub(crate) context: RumeContext,
}
