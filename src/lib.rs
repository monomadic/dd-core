extern crate vst2;

#[macro_use] 
extern crate log;
extern crate simplelog;

#[macro_use] extern crate glium;

extern crate winit;
extern crate glutin;

pub mod vst;
pub mod util;
pub mod gui;
pub mod widgets;
mod base;

// external objects
pub use vst2::*;
pub use vst2::plugin::{ HostCallback, Category };
pub use vst2::buffer::AudioBuffer;
pub use vst2::host::Host;
pub use base::config::PluginConfig;
pub use base::param::Param;
pub use base::BasePlugin;
pub use gui::{ Graphics };
// pub use gui::widget_id;

// internal objects
//use vst::VSTPlugin;

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
