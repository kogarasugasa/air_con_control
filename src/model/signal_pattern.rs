use crate::model::Signal;
pub struct SignalPattern {
    pub name: String,
    pub signals: Vec<Signal>,
}

impl SignalPattern {
    pub fn duration(&self) -> std::time::Duration {
        self.signals.iter().map(|sig| sig.elapsed).sum()
    }
}