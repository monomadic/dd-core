pub mod window;
pub mod events;

pub use self::window::Window;

#[derive(Debug)]
pub enum GUIError {
	CreationError(String),
}
