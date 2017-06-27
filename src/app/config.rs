use vst2::plugin::HostCallback;

use app::params::*;

#[derive(Default)]
pub struct PluginConfig {
    pub name: String,
    pub vendor: String,
    pub host: HostCallback,
    pub params: Vec<Param>,
}
