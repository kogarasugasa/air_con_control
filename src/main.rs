use rppal::gpio;
use std::{thread, time};
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
    while true {
        let pin_value = pin.read();
        println!("Pin value: {:?}", pin_value);
        thread::sleep(time::Duration::from_millis(50));
    }
}
