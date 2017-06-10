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
    // pub fn add_param(&mut self, param: Param) {
    //     self.params.
    // }

    // pub fn new() -> Self {
    //     AppConfig {
    //         params: params(),
    //         host: Default::default(),
    //     }
    // }
}

// impl AppConfig {
//     pub fn get_parameter_name(&self, index: usize) -> String {
//         self.params[index].name.clone()
//     }
// }
