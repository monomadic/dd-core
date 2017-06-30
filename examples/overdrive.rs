extern crate dd_core;
use dd_core::*;
use dd_core::conrod;
use dd_core::conrod::widget::*;

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
            unique_id: 222666,
            inputs: 2,
            outputs: 2,
            category: Category::Effect,
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

impl Graphics for TestPlugin {
    fn setup_ids(&mut self, generator: &mut conrod::widget::id::Generator) -> Vec<conrod::widget::Id> {
        vec![
            generator.next(),
            generator.next(),
            generator.next(),
        ]
    }

    fn do_layout(&mut self, ref mut ui: conrod::UiCell, config: &mut PluginConfig, ids: &mut Vec<conrod::widget::Id>) {
        use conrod::{color, Labelable, Colorable, Sizeable, Widget, Borderable, Positionable};
        use conrod::widget::Canvas;

        let border_width = 1.0;

        // background
        Canvas::new()
            .color(color::Color::Rgba(0.1, 1.0, 0.1, 1.0))
            .set(ids[0], ui);

        // gain_slider
        if let Some(val) = Slider::new(config.params[0].value, 0.0, 1.0)
            .w_h(300.0, 30.0)
            .x_y(0.0, 50.0)
            .color(color::LIGHT_BLUE)
            .border(border_width)
            // .label(&label)
            .label_color(color::WHITE)
            .set(ids[1], ui) {
                config.params[0].value = val;
                config.host.automate(0 as i32, config.params[0].value);
            }

        // threshold_slider
        if let Some(val) = Slider::new(config.params[1].value, 0.0, 1.0)
            .w_h(300.0, 30.0)
            .x_y(0.0, -50.0)
            .color(color::LIGHT_PURPLE)
            .border(border_width)
            // .label(&label)
            .label_color(color::WHITE)
            .set(ids[2], ui) {
                config.params[1].value = val;
                // info!("vst version: {:?}", config.host.vst_version());
                config.host.automate(1 as i32, config.params[1].value);
            }
    }
}

create_plugin!(TestPlugin);
