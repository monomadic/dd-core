pub struct Param {
    pub name: String,
    pub value: f32,
}

pub struct Params {
    pub params: Vec<Param>,
}

impl Default for Params {
    fn default() -> Self {
        Params{
            params: vec![
                Param{ name: "Gain".to_string(), value: 0.001 },
                Param{ name: "Threshold".to_string(), value: 0.001 },
            ]
        }
    }
}
