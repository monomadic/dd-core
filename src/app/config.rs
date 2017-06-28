use vst2::plugin::HostCallback;

use app::params::*;

use Category;

pub struct PluginConfig {
    pub name: String,
    pub vendor: String,
    pub host: HostCallback,
    pub params: Vec<Param>,
    pub unique_id: i32,
    pub inputs: i32,
    pub outputs: i32,
    pub category: Category,
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig {
            category: Category::Effect,
            inputs: 2,
            outputs: 2,

            ..Default::default()
        }
    }
}