mod editor;
pub mod plugin;
use gui::Window;
use app::config::*;
use vst2::plugin::HostCallback;

use Graphics;

#[derive(Default)]
pub struct VSTPlugin<P> where P: BasePlugin + Graphics {
    pub window: Option<Window>,
	pub plugin: P,
	config: PluginConfig,
}

pub trait BasePlugin where Self : Sized {
	fn new(HostCallback) -> (Self, PluginConfig);
}
