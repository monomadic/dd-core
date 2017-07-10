pub mod window;

pub use self::window::Window;
// use BasePlugin;

#[derive(Debug)]
pub enum GUIError {
	CreationError(String),
}

// // use conrod;
// use PluginConfig;
pub trait Graphics {
}

// // use conrod::text::{Font};
// // pub struct GraphicsConfig {
// //     pub widget_ids: Vec<String>,
// //     pub theme: conrod::Theme,
// //     pub fonts: Font,
// // }

// // pub fn widget_id(ids: &mut HashMap<String, conrod::widget::Id>, id: &str)  -> conrod::widget::Id {
// //     *ids.get(id).unwrap()
// // }