pub mod param;
pub mod config;

use PluginConfig;
use HostCallback;

pub trait BasePlugin where Self : Sized {
    fn new(HostCallback) -> (Self, PluginConfig);
}
