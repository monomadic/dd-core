mod editor;
pub mod plugin;
use gui::Window;

// use vst2::editor::Editor;

use PluginConfig;
// use HostCallback;
use BasePlugin;
// use Graphics;

#[derive(Default)]
pub struct VSTPlugin<P:BasePlugin> {
    pub window: Option<Window>,
	pub plugin: P,
	config: PluginConfig,
}
