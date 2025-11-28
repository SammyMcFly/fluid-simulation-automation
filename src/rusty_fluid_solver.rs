use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct Setup {
    pub parameters: HashMap<String, toml::Value>,
    pub light: HashMap<String, toml::Value>,
    pub scene: HashMap<String, toml::Value>,
}