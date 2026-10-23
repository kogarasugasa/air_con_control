use std::thread;
use std::time::Duration;
use rppal::gpio;

pub struct Button {
    pub pin: gpio::InputPin,
    pub on_level: gpio::Level,
}
impl Button {
    pub fn is_on(&self) -> bool {
        self.pin.read() == self.on_level
    }
    pub fn is_off(&self) -> bool {
        self.pin.read() != self.on_level
    }
    pub fn wait(&self, timeout: Duration, pool_span: Duration) {
        let start_time = std::time::Instant::now();
        loop {
            if self.pin.read() == self.on_level {
                break;
            }
            if start_time.elapsed() > timeout {
                break;
            }
            thread::sleep(pool_span);
        }
    }
}
