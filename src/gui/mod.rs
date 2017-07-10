pub mod window;
pub use self::window::Window;

#[derive(Debug)]
pub enum GUIError {
	CreationError(String),
}

// use conrod;
pub trait Graphics {
}
