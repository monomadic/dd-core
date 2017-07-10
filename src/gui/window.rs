use std::os::raw::c_void;

use gui::GUIError;

use PluginConfig;
use Graphics;

pub struct Window {}

impl Window {
    pub fn new<P:Graphics>(handle: *mut c_void, plugin: &mut P) -> Result<Window, GUIError> {
        Err(GUIError::CreationError("not implemented".to_string()))
    }
}
