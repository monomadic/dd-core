extern crate dd_core;
use dd_core::*;
use std::collections::HashMap;

#[derive(Default)]
struct TestPlugin {}

impl BasePlugin for TestPlugin {
    fn new(host: HostCallback) -> (Self, PluginConfig) {(
        TestPlugin {
        },
        PluginConfig {
            name: "DDOverdrive".to_string(),
            vendor: "DeathDisco".to_string(),
            host: host,
            unique_id: 222666,
            inputs: 2,
            outputs: 2,
            category: Category::Effect,
            gui: None
            params: vec![
                Param{ name: "Gain".to_string(), value: 0.001 },
                Param{ name: "Threshold".to_string(), value: 0.001 },
            ],
        })
    }

    fn process_dsp(&mut self, buffer: AudioBuffer<f32>, config: &mut PluginConfig) {
      // Split out the input and output buffers into two vectors
        let (inputs, outputs) = buffer.split();

        // For each buffer, transform the samples
        for (input_buffer, output_buffer) in inputs.iter().zip(outputs) {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer) {

                if *input_sample >= 0.0 {
                    *output_sample = input_sample.min(config.params[1].value) / config.params[1].value * config.params[0].value;
                }
                else {
                    *output_sample = input_sample.max(-config.params[1].value) / config.params[1].value * config.params[0].value;
                }

            }
        }
    }
}

create_plugin!(TestPlugin);
