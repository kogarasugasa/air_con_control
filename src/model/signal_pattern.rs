use crate::model::signal;

pub struct SignalPattern {
    pub name: String,
    pub signals: Vec<signal>,
}

impl SignalPattern {
    pub fn duration(&self) -> std::time::Duration {
        self.signals.iter().map(|sig| sig.elapsed).sum()
    }
}