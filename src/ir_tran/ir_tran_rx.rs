use rppal::gpio::{ self, Pin, Level };
use std::{thread, time};
use super::model::Signal;

pub fn receive_pattern(pin: Pin) -> Vec<Signal> {
    let mut pattern = vec![];
    while true {
        let val = pin.read();
        if val == gpio::Level::High {
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
        thread::sleep(time::Duration::from_millis(1));
    }
    pattern
}
