use rppal::gpio;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PinLevel {
    High,
    Low
}
impl From<gpio::Level> for PinLevel {
    fn from(value: gpio::Level) -> Self {
        match value {
            gpio::Level::High => Self::High,
            gpio::Level::Low => Self::Low,
        }
    }
}