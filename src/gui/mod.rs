pub mod window;
pub mod events;

pub use self::window::Window;
use ::BasePlugin;

#[derive(Debug)]
pub enum GUIError {
	CreationError(String),
}
