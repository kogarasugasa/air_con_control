use rppal::gpio::{ self, Pin, Level };
use serde_json::from_reader;
use std::{thread, time};
use crate::model::Signal;

pub fn receive_pattern(pin: Pin) -> Vec<Signal> {
    let mut pattern = vec![];
    let init_val = pin.read();
    while true {
        let val = pin.read();
        if val != init_val {
            break;
        }
        thread::sleep(time::Duration::from_millis(1));
    }
    let receive_timer = thread::spawn(move || thread::sleep(time::Duration::from_secs(5)));
    let std_time = time::Instant::now();
    while receive_timer.is_finished() == false {
        let signal = Signal {
            level: pin.read(),
            elapsed: std_time.elapsed(),
        };
        pattern.push(signal);
        thread::sleep(time::Duration::from_millis(1));
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
