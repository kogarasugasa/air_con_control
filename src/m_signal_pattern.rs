use std::time;
use serde::{Serialize, Deserialize};

use crate::m_signal::Signal;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPattern {
    pub name: String,
    pub signals: Vec<Signal>,
}
impl SignalPattern {
    fn duration(&self) -> time::Duration {
        self.signals.iter().map(|sig| sig.elapsed).sum()
    }
}