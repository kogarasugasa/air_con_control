use rppal::gpio::{ self, Pin, Level };
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
            value: pin.read(),
            elapsed: std_time.elapsed(),
        };
        pattern.push(signal);
        thread::sleep(time::Duration::from_millis(10));
    }
    pattern
}
