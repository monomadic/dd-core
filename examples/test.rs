extern crate dd_core;
use dd_core::*;

#[derive(Default)]
struct TestPlugin {}

impl BasePlugin for TestPlugin {
	fn new(host: HostCallback) -> (Self, PluginConfig) {(
		TestPlugin {
		},
		PluginConfig {
            name: "DDTestPlugin".to_string(),
            vendor: "DeathDisco".to_string(),
            host: host,
            params: vec![
                Param{ name: "Gain".to_string(), value: 0.001 },
                Param{ name: "Threshold".to_string(), value: 0.001 },
            ]
		})
	}
}

create_plugin!(TestPlugin);
