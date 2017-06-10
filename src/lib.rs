#[macro_use] 
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;
// extern crate log_panics;

extern crate libc;

#[macro_use] extern crate conrod;

// extern crate find_folder;
// extern crate image;

extern crate winit;

mod vst;
mod app;
mod window;

use vst::plugin::VSTPlugin;
plugin_main!(VSTPlugin);
