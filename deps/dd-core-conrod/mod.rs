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
    fn get_config(&mut self) -> GraphicsConfig;
    fn setup_display(&mut self, window: &mut Window);
    fn do_layout(&mut self, ui: conrod::UiCell, config: &mut PluginConfig, ids: &mut HashMap<String, conrod::widget::Id>);
}

use conrod::text::{Font};
pub struct GraphicsConfig {
    pub widget_ids: Vec<String>,
    pub theme: conrod::Theme,
    pub fonts: Font,
}

pub fn widget_id(ids: &mut HashMap<String, conrod::widget::Id>, id: &str)  -> conrod::widget::Id {
    *ids.get(id).unwrap()
}