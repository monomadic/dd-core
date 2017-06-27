#[macro_use] 
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

#[macro_use] extern crate conrod;

extern crate winit;

pub mod vst;
mod app;
mod gui;

pub use vst2::*;
pub use vst2::plugin::HostCallback;
pub use app::config::PluginConfig;
pub use vst::BasePlugin;
pub use app::params::Param;
use vst::VSTPlugin;

#[macro_export]
macro_rules! create_plugin {
    ($config:ty) => {
    	// info!("starting VST plugin.");
        plugin_main!(vst::VSTPlugin<$config>);
    }
}
