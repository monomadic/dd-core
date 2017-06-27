use vst2::plugin::HostCallback;

use app::params::*;

#[derive(Default)]
pub struct PluginConfig {
    pub name: String,
    pub vendor: String,
    pub host: HostCallback,
    pub params: Vec<Param>,
}

// pub trait DDPlug {}

// impl AppConfig {
//     pub fn new(host: HostCallback) -> Self {
//         AppConfig {
//             name: "DDGuiExample".to_string(),
//             vendor: "DeathDisco".to_string(),
//             host: host,
//             params: vec![
//                 Param{ name: "Gain".to_string(), value: 0.001 },
//                 Param{ name: "Threshold".to_string(), value: 0.001 },
//             ]
//         }
//     }
// }
