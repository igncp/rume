pub trait ConfigCompilerPlugin {}

pub struct AutoPatchConfigPlugin;
pub struct DefaultConfigPlugin;
pub struct LegacyPresetConfigPlugin;
pub struct LegacyDictionaryConfigPlugin;
pub struct BuildInfoPlugin;
pub struct SaveOutputPlugin;

impl ConfigCompilerPlugin for AutoPatchConfigPlugin {}
impl ConfigCompilerPlugin for DefaultConfigPlugin {}
impl ConfigCompilerPlugin for LegacyPresetConfigPlugin {}
impl ConfigCompilerPlugin for LegacyDictionaryConfigPlugin {}
impl ConfigCompilerPlugin for BuildInfoPlugin {}
impl ConfigCompilerPlugin for SaveOutputPlugin {}
