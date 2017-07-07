#[macro_use]
extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

pub mod vst;
mod base;

// external objects
pub use vst2::*;
pub use vst2::plugin::{ HostCallback, Category };
pub use vst2::buffer::AudioBuffer;
pub use vst2::host::Host;
pub use vst2::editor::Editor;
pub use base::config::PluginConfig;
pub use base::param::Param;
pub use base::BasePlugin;

// internal objects
use vst::VSTPlugin;

#[macro_export]
macro_rules! create_plugin {
    ($config:ty) => {
    	// info!("starting VST plugin.");
        plugin_main!(vst::VSTPlugin<$config>);
    }
}

#[macro_export]
macro_rules! hashmap {
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $( m.insert($key, $value); )+ m
        }
    };
}

#[macro_export]
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
