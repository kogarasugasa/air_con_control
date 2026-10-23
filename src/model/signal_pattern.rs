use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{Read, Write};

use crate::model::{
    signal::Signal,
    pin_level::PinLevel,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPattern {
    pub name: String,
    pub signals: Vec<Signal>,
}
impl SignalPattern {
    pub fn reverse_phase(&mut self) {
        for signal in self.signals.iter_mut() {
            match signal.level {
                PinLevel::High => signal.level = PinLevel::Low,
                PinLevel::Low => signal.level = PinLevel::High,
            }
        }
    }
    pub fn save(&self, file_path: &str) -> Result<(), String> {
        let file = fs::OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .open(file_path);
        let mut file = match file {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        let json = match serde_json::to_string(self) {
            Ok(json) => json,
            Err(e) => return Err(e.to_string())
        };
        if let Err(e) = file.write(json.as_bytes()) {
            return Err(e.to_string());
        };
        Ok(())
    }
    pub fn read(file_path: &str) -> Result<Self, String> {
        let mut file = match fs::File::options()
            .read(true)
            .open(file_path) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string())
        };
        let mut json = String::new();
        if let Err(e) = file.read_to_string(&mut json) {
            return Err(e.to_string());
        }
        let pattern: SignalPattern = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string())
        };
        Ok(pattern)
    }
}