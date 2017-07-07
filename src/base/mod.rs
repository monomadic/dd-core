pub mod param;
pub mod config;

use vst2::editor::Editor;
use PluginConfig;
use HostCallback;
use AudioBuffer;

pub trait BasePlugin where Self : Sized {
    fn new(HostCallback) -> (Self, PluginConfig);
    fn process_dsp(&mut self, buffer: AudioBuffer<f32>, config: &mut PluginConfig);
    fn get_editor(&mut self) -> Option<&mut Editor>;
}
