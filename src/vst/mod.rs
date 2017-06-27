mod editor;
pub mod plugin;
use gui::Window;
use app::config::*;
use vst2::plugin::HostCallback;

#[derive(Default)]
pub struct VSTPlugin<P> where P : BasePlugin {
    // threshold: f32,
    // gain: f32,
    pub window: Option<Window>,
    // pub config: AppConfig,
	pub plugin: P,
	config: PluginConfig,
}

pub trait BasePlugin where Self : Sized {
	fn new(HostCallback) -> (Self, PluginConfig);
}
