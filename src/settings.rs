use serde::{Serialize, Deserialize};
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub rendering: String,
    pub fov: f32,
    pub window_size: u32,
    pub vertex_size: f32,
    pub rotation_incr: f64,
}

impl Settings {
    pub fn store(&self) {
        let serialized = serde_json::to_string(self).unwrap();
        fs::write("settings.json", serialized).expect("Unable to write settings.");
    }

    pub fn load() -> Settings {
        let mut data = String::new();
        let mut file = File::open("settings.json").unwrap();
        file.read_to_string(&mut data).unwrap();

        let s: Settings = serde_json::from_str(&data).expect("Unable to read");

        s
    }
}