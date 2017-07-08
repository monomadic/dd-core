pub mod plugin;
use PluginConfig;
// use HostCallback;
use BasePlugin;

#[derive(Default)]
pub struct VSTPlugin<P> where P: BasePlugin {
	pub plugin: P,
	config: PluginConfig,
}
