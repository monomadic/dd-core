pub mod window;
pub mod events;

use std::collections::HashMap;

pub use self::window::Window;
use BasePlugin;

#[derive(Debug)]
pub enum GUIError {
	CreationError(String),
}

use conrod;
use PluginConfig;
pub trait Graphics {
    fn widget_ids(&mut self) -> Vec<String>;
    fn setup_display(&mut self, window: &mut Window);
    fn do_layout(&mut self, ui: conrod::UiCell, config: &mut PluginConfig, ids: &mut HashMap<String, conrod::widget::Id>);
}

pub fn widget_id(ids: &mut HashMap<String, conrod::widget::Id>, id: &str)  -> conrod::widget::Id {
    *ids.get(id).unwrap()
}