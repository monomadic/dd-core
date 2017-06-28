pub mod window;
pub mod events;

pub use self::window::Window;
use BasePlugin;

#[derive(Debug)]
pub enum GUIError {
	CreationError(String),
}

use conrod;
use PluginConfig;
pub trait Graphics {
    fn setup_ids(&mut self, generator: &mut conrod::widget::id::Generator) -> Vec<conrod::widget::Id>;
    fn do_layout(&mut self, ui: conrod::UiCell, config: &mut PluginConfig, ids: &mut Vec<conrod::widget::Id>);
}
