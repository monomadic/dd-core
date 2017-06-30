pub mod param;
pub mod config;

use PluginConfig;
use HostCallback;
use AudioBuffer;

pub trait BasePlugin where Self : Sized {
    fn new(HostCallback) -> (Self, PluginConfig);
    fn process_dsp(&mut self, buffer: AudioBuffer<f32>, config: &mut PluginConfig);
}
