#[macro_use] 
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

#[macro_use] extern crate conrod;

// extern crate find_folder;
// extern crate image;

extern crate winit;

pub mod vst;
mod app;
mod gui;

pub use vst2::*;

#[macro_export]
macro_rules! create_plugin {
    ($plugin:ty) => {
        plugin_main!($plugin);
    }
}
