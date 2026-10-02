use std::{thread, time};
use rppal::gpio;

use crate::m_signal::Signal;
use crate::m_signal_pattern::SignalPattern;
use crate::m_pin_level::PinLevel;

pub fn send_pattern(pin: gpio::Pin, pattern: SignalPattern) {
    let mut out_pin = pin.into_output();
    for i in 0..pattern.signals.len() {
        let signal = &pattern.signals[i];
        let before = if i == 0 {
            time::Duration::ZERO
        }
        else {
            pattern.signals[i - 1].elapsed
        };
        thread::sleep(signal.elapsed - before);
        match signal.level {
            PinLevel::High => {
                out_pin.set_high();
            },
            PinLevel::Low => {
                out_pin.set_low();
            }
        }
    }
}
pub fn receive_pattern(pin: gpio::Pin) -> Vec<Signal> {
    let mut pattern = Vec::with_capacity(12000);
    let init_val = pin.read();
    while pin.read() == init_val {
        thread::sleep(time::Duration::from_millis(1));
    }
    let receive_timer = thread::spawn(move || thread::sleep(time::Duration::from_secs(5)));
    let std_time = time::Instant::now();
    while !receive_timer.is_finished() {
        let signal = Signal {
            level: pin.read().into(),
            elapsed: std_time.elapsed(),
        };
        pattern.push(signal);
        thread::sleep(time::Duration::from_micros(10));
    }
    let mut before_signal = match pattern.first() {
        Some(v) => v,
        None => return vec![]
    };
    let mut compress = vec![before_signal.clone()];
    for i in 1..pattern.len() {
        let signal = &pattern[i];
        let span = signal.elapsed - before_signal.elapsed;
        if span.as_millis() >= 1000 {
            break;
        }
        if before_signal.level != signal.level {
            before_signal = signal;
            compress.push(signal.clone());
        }
    }
    compress
}
pub fn get_pin(num: u8) -> Result<gpio::Pin, gpio::Error> {
    let pin = gpio::Gpio::new()?.get(num)?;
    Ok(pin)
}