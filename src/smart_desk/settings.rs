use std::fs;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistentData {
    upper_height: u32,
    lower_height: u32,
}

impl PersistentData {
    pub fn lower_height(&self) -> u32 {
        self.lower_height
    }
    pub fn upper_height(&self) -> u32 {
        self.upper_height
    }
    pub fn new() -> PersistentData {
        PersistentData {
            upper_height: 0,
            lower_height: 0,
        }
    }
    pub fn set_persistent_data(&self, data_path: &str) -> Result<(), Box<dyn Error>> {
        let data = toml::to_string(self)?;
        fs::write(data_path, data)?;
        Ok(())
    }
    pub fn get_persistent_data() -> Result<PersistentData, Box<dyn Error>> {
        let data_path = "/home/desk1/settings";
        if let Ok(contents) = fs::read_to_string(data_path) {
            if let Ok(data) = toml::from_str(&contents.as_str()) {
                return Ok(data);
            }
        }
        // corruption occured? rewrite empty data
        println!("Rewriting new persistent data!");
        let data = PersistentData::new();
        data.set_persistent_data(data_path)?;
        return Ok(data);
    }
}

