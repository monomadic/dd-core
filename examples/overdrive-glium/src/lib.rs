extern crate dd_core;
use dd_core::*;
use std::os::raw::c_void;

extern crate dd_core_glium;
use dd_core_glium::{ Window, WindowConfig };

#[derive(Default)]
struct TestPlugin {
    window: Option<Window>,
}

impl BasePlugin for TestPlugin {
    fn new(host: HostCallback) -> (Self, PluginConfig) {(
        TestPlugin {
            window: Some(Window::new()),
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
    fn get_editor(&mut self) -> Option<&mut Editor> {
        Some(self)
    }
}

impl Editor for TestPlugin {
    fn size(&self) -> (i32, i32) { (500, 300) }
    fn position(&self) -> (i32, i32) { (0, 0) }
    fn is_open(&mut self) -> bool { self.window.is_some() }
    fn close(&mut self) { self.window = None; }

    fn open(&mut self, handle: *mut c_void) {
        self.window = Window::new(handle);
        // match Window::new(window as *mut c_void, &mut self.plugin) {
        //     Ok(w) => {
        //         self.window = Some(w);
        //     },
        //     Err(why) => { panic!("{:?}", why) }
        // }
    }

    fn idle(&mut self) {
        if let Some(ref mut window) = self.window {
            window.draw(&mut self.config, &mut self.plugin);
        }
    }
}

create_plugin!(TestPlugin);
