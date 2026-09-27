mod ir_tran_rx;

use ir_tran_rx::ir_tran_rs::receive_pattern;
use rppal::gpio;
use std::thread::JoinHandle;
use std::{thread, time};

pub fn main() {
    let pin = match gpio::Gpio::new() {
        Ok(gpio) => gpio.get(4),
        Err(e) => {
            eprintln!("Failed to access GPIO: {}", e);
            return;
        }
    };
    let pin = match pin {
        Ok(pin) => pin,
        Err(e) => {
            eprintln!("Failed to get GPIO pin: {}", e);
            return;
        }
    };
    let pattern = receive_pattern(pin);
    for signal in pattern {
        println!(
            "Signal: {:?}, Elapsed: {:?}",
            signal.value, signal.elapsed
        );
    }
}
struct SignalPattern {
    name: String,
    signals: Vec<Signal>,
    duration: time::Duration,
}