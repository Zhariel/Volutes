use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub rendering: String,
    pub fov: usize,
}

impl Settings {
    pub fn store(&self){
        let filename = "settings.json";
        let serialized = serde_json::to_string(self).unwrap();
        fs::write(filename, serialized).expect("Unable to write settings.");
    }

    pub fn load() -> Settings{
        let filename = "settings.json";

        let mut data = String::new();
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut data).unwrap();

        let s: Settings = serde_json::from_str(&data).expect("Unable to read");

        s
    }
}