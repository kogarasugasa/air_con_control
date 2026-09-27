use rppal::gpio;
use std::thread::JoinHandle;
use std::rc::Rc;
use std::{pin, thread, time};
fn main() {
    println!("Hello, world!");
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
fn receive_pattern(pin: gpio::Pin) -> Vec<Signal> {
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

struct SignalPattern {
    name: String,
    signals: Vec<Signal>,
    duration: time::Duration,
}
struct Signal {
    value: gpio::Level,
    elapsed: time::Duration,
}