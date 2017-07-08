extern crate dd_core;
use std::os::raw::c_void;

#[derive(Default)]
pub struct Window {
    window: Option<i32>,
}

impl Window {
    pub fn new(handle: *mut c_void) -> Self {
        Window {
            window: None,
        }
    }
}
