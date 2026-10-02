use serde::{Serialize, Deserialize};

use crate::{m_pin_level::PinLevel, m_signal::Signal};

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
}