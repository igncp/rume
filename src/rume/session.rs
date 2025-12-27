use crate::rume::engine::Engine;

pub type RumeSessionId = usize;

pub struct RumeSession {
    pub(super) id: RumeSessionId,
    pub(super) engine: Engine,
    pub(super) commited_text: String,
}
