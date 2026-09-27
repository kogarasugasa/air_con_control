use rppal::gpio;
use std::time;

pub struct Signal {
    pub value: gpio::Level,
    pub elapsed: time::Duration,
}