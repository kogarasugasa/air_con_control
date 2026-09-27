use rppal::gpio;
use std::time;

[derive(clone)]
pub struct Signal {
    pub level: gpio::Level,
    pub elapsed: time::Duration,
}