use std::time;
use serde::{Serialize, Deserialize};

use crate::m_pin_level::PinLevel;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signal {
    pub level: PinLevel,
    pub elapsed: time::Duration,
}