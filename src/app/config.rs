use vst2::plugin::HostCallback;

use app::params::*;

#[derive(Default)]
pub struct AppConfig {
    pub host: HostCallback,
    pub params: Params,
}

impl AppConfig {
    pub fn new(host: HostCallback) -> Self {
        AppConfig {
            host: host,
            params: Params::default(),
        }
    }
}
