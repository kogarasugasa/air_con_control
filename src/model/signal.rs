use rppal::gpio;
use std::time;

#[derive(Clone)]
pub struct Signal {
    pub level: gpio::Level,
    pub elapsed: time::Duration,
}