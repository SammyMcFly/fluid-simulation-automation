use serde::Deserialize;
use std::collections::HashMap;
use itertools::Itertools;


#[derive(Debug, Deserialize)]
pub struct MeasurementConfig {
    pub general: General,
    pub operation: Operation,
    pub parameters: HashMap<String, toml::Value>,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub executable: String,
    pub config_file: String,
    pub state_file: String,
    pub measurement_destination_file_path: String,
    pub start_time: f64,
    pub finish_time: f64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "parameters")]
pub enum Operation {
    Zip,
    Cartesian,
}

impl MeasurementConfig {
    pub fn parse_from_file(file_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        // Read file into a string
        let content_string = match std::fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => panic!("Could not read measurement file: {}", e)
        };
        // Parse
        let measurement_config: Self = toml::from_str(&content_string)?;
        Ok(measurement_config)
    }
    pub fn get_measurement_series(&self) -> Vec<HashMap<String, toml::Value>> {
        match self.operation {
            Operation::Zip => {
                let min_len = self.parameters.values().map(|v| {
                    v.as_array().expect("Parameters must be defined in an array").len()
                }).min().unwrap();
                let mut parameter_combinations = Vec::with_capacity(min_len);
                for i in 0..min_len {
                    let mut combination = HashMap::new();
                    for (key, array) in &self.parameters {
                        combination.insert(key.clone(), array[i].clone());
                    }
                    parameter_combinations.push(combination);
                }
                parameter_combinations
            },
            Operation::Cartesian => {
                let keys: Vec<String> = self.parameters.keys().cloned().collect();
                // Get a Vec of iterators over each value list
                let list_iters: Vec<Vec<toml::Value>> = keys
                    .iter()
                    .map(|k| self.parameters[k].as_array().expect("Parameters must be defined in an array").clone())
                    .collect();
                let mut parameter_combinations = Vec::new();
                for value_combi in list_iters.into_iter().multi_cartesian_product() {
                    let mut combination = HashMap::new();
                    for (key, val) in keys.iter().zip(value_combi.into_iter()) {
                        combination.insert(key.clone(), val.clone());
                    }
                    parameter_combinations.push(combination);
                }
                parameter_combinations
            },
        }
    }
}