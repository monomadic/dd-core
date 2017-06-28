use conrod;

use vst2::plugin::HostCallback;
use vst2::host::Host;

use BasePlugin;
use PluginConfig;

use conrod::widget::Id;

pub trait Graphics {
	fn setup_ids(&mut self, generator: &mut conrod::widget::id::Generator) -> Vec<conrod::widget::Id>;
	fn do_layout(&mut self, ui: conrod::UiCell, config: &mut PluginConfig, ids: &mut Vec<conrod::widget::Id>);
}
