#[macro_use] 
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

#[macro_use]
pub extern crate conrod;

extern crate winit;

pub mod vst;
mod base;
mod gui;

// external objects
pub use vst2::*;
pub use vst2::plugin::{ HostCallback, Category };
pub use vst2::buffer::AudioBuffer;
pub use vst2::host::Host;
pub use base::config::PluginConfig;
pub use base::param::Param;
pub use base::BasePlugin;
pub use gui::Graphics;

// internal objects
use vst::VSTPlugin;

#[macro_export]
macro_rules! create_plugin {
    ($config:ty) => {
    	// info!("starting VST plugin.");
        plugin_main!(vst::VSTPlugin<$config>);
    }
}
