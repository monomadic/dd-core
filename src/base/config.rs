use HostCallback;
use Category;
use Param;

pub struct PluginConfig {
    pub name: String,
    pub vendor: String,
    pub host: HostCallback,
    pub params: Vec<Param>,
    pub unique_id: i32,
    pub inputs: i32,
    pub outputs: i32,
    pub category: Category,
    pub size: (i32, i32),
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig {
            category: Category::Effect,
            inputs: 2,
            outputs: 2,
            host: Default::default(),
            name: "Default Plugin".to_string(),
            params: Vec::new(),
            unique_id: 00011112,
            vendor: "DDCore".to_string(),
            size: (360,160),
        }
    }
}