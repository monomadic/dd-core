use vst2::buffer::AudioBuffer;
use vst2::plugin::{ Plugin, Info, HostCallback };
use vst2::editor::Editor;
// use vst2::host::Host;

use simplelog;
use std::fs::File;

use vst::{ VSTPlugin };

// use PluginConfig;
use BasePlugin;

impl<P> Plugin for VSTPlugin<P> where P: BasePlugin {
    fn new(host: HostCallback) -> Self {

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        let _ = simplelog::CombinedLogger::init(
            vec![
                simplelog::WriteLogger::new(
                    simplelog::LogLevelFilter::Info,
                    simplelog::Config::default(),
                    File::create("/tmp/simplesynth.log").expect("log to open correctly.")),
            ]);

        let (plugin, config) = P::new(host);

        VSTPlugin {
            plugin: plugin,
            config: config,
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: self.config.name.clone(),
            vendor: self.config.vendor.clone(),
            unique_id: self.config.unique_id,
            category: self.config.category,
            inputs: self.config.inputs,
            outputs: self.config.outputs,
            parameters: self.config.params.len() as i32,

            ..Info::default()
        }
    }

    fn can_be_automated(&self, _: i32) -> bool { true }

    fn get_editor(&mut self) -> Option<&mut Editor> {
        self.plugin.get_editor()
        // if self.editor.is_some() {
        //     Some(self.editor)
        // } else {
        //     None
        // }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.config.params[index as usize].value
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        self.config.params[index as usize].value = value.max(0.01);
    }

    fn get_parameter_name(&self, index: i32) -> String {
        self.config.params[index as usize].name.clone()
    }

    fn get_parameter_text(&self, index: i32) -> String {
        format!("{}", self.config.params[index as usize].value * 100.0)
    }

    fn get_parameter_label(&self, _: i32) -> String {
        "%".to_string()
    }

    fn process(&mut self, buffer: AudioBuffer<f32>) {
        self.plugin.process_dsp(buffer, &mut self.config);
    }
}
