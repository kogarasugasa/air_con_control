mod ir_tran_rx;
use ir_tran_rx::receive_pattern;
use std::{str::pattern::Pattern, time};
use rppal::gpio;
use crate::model::Signal;
use crate::model::SignalPattern;
use std::fs;

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
    let pattern = SignalPattern {
        name: String::from("Example Pattern"),
        signals: pattern,
    };
    match save_pattern_to_file(&pattern, "signal_pattern.txt") {
        Ok(_) => println!("Pattern saved to signal_pattern.txt"),
        Err(e) => eprintln!("Failed to save pattern: {}", e),
    }
}
fn save_pattern_to_file(pattern: &SignalPattern, file_path: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    writeln!(file, "Pattern Name: {}", pattern.name)?;
    for signal in &pattern.signals {
        writeln!(file, "Signal: {:?}, Elapsed: {:?}", signal.value, signal.elapsed)?;
    }
    Ok(())
}