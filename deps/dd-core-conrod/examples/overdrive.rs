extern crate dd_core;
use dd_core::*;
use dd_core::widgets::*;
use dd_core::conrod;
use dd_core::conrod::widget::*;
use dd_core::conrod::text::{FontCollection};

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

use conrod::position::{Align, Direction, Padding, Position, Relative};
impl Graphics for TestPlugin {
    fn get_config(&mut self) -> GraphicsConfig {
        GraphicsConfig {
            widget_ids: string_vec! [
                "body",
                "title",
                "knob",
                "gain_slider",
                "threshold_slider"
            ],
            theme: conrod::Theme {
                name: "DeathDisco".to_string(),
                padding: Padding::none(),
                x_position: Position::Relative(Relative::Align(Align::Start), None),
                y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
                background_color: conrod::color::DARK_CHARCOAL,
                shape_color: conrod::color::LIGHT_CHARCOAL,
                border_color: conrod::color::BLACK,
                border_width: 1.0,
                label_color: conrod::color::WHITE,
                font_id: None,
                font_size_large: 26,
                font_size_medium: 18,
                font_size_small: 12,
                widget_styling: conrod::theme::StyleMap::default(),
                mouse_drag_threshold: 0.0,
                double_click_threshold: std::time::Duration::from_millis(500),
            },
            fonts: FontCollection::from_bytes(include_bytes!("assets/Roboto-Light.ttf") as &[u8]).into_font().unwrap(),
        }
    }

    fn setup_display(&mut self, _: &mut dd_core::gui::Window) {}

    fn do_layout(&mut self, ref mut ui: conrod::UiCell, config: &mut PluginConfig, ids: &mut HashMap<String, conrod::widget::Id>) {
        use conrod::{color, Labelable, Colorable, Sizeable, Widget, Positionable};
        use conrod::widget::Canvas;
    
        // background
        Canvas::new()
            .color(color::Color::Rgba(0.1, 0.1, 0.1, 1.0))
            .set(widget_id(ids, "body"), ui);

        Text::new("ddOverdrive")
            .top_left_of(widget_id(ids, "body"))
            .color(conrod::color::WHITE)
            .font_size(12)
            .set(widget_id(ids, "title"), ui);

        Knob::new()
            .set(widget_id(ids, "knob"), ui);

        // gain_slider
        if let Some(val) = Slider::new(config.params[0].value, 0.0, 1.0)
            .w_h(300.0, 30.0)
            .x_y(0.0, 50.0)
            .color(color::LIGHT_BLUE)
            .label("GAIN")
            .set(widget_id(ids, "gain_slider"), ui) {
                config.params[0].value = val;
                config.host.automate(0 as i32, config.params[0].value);
            }

        // threshold_slider
        if let Some(val) = Slider::new(config.params[1].value, 0.0, 1.0)
            .w_h(300.0, 30.0)
            .x_y(0.0, -50.0)
            .color(color::LIGHT_PURPLE)
            .label("THRESHOLD")
            .set(widget_id(ids, "threshold_slider"), ui) {
                config.params[1].value = val;
                config.host.automate(1 as i32, config.params[1].value);
            }
    }
}

create_plugin!(TestPlugin);
