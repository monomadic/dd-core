extern crate dd_core;
use dd_core::*;

struct OverdrivePlugin;

impl Config for OverdrivePlugin {
    pub fn new() -> Self {
        PluginConfig {
            name: "DD-Overdrive".to_string(),
            vendor: "deathdisco.wtf".to_string(),
            unique_id: 222666,
            version: 0001,
            inputs: 2,
            outputs: 2,
            parameters: 2,
            category: Effect,
            params: vec![
                Param{ name: "Gain".to_string(), value: 0.001 },
                Param{ name: "Threshold".to_string(), value: 0.001 },
            ]
        }
    }
}

impl Processor for OverdrivePlugin {
    pub fn process() {}
    pub fn midi_events() {}
}

impl GUI for OverdrivePlugin {
    pub fn draw() {}
    pub fn ui_events() {}
}
