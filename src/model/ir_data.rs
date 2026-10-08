use std::time::{Duration, Instant};
use rppal::gpio::{OutputPin};

use crate::m_pin_level::PinLevel;
use crate::m_signal_pattern::SignalPattern;
use crate::m_signal::Signal;
pub struct IrData {
    pub name: String,
    pub bits: Vec<PinLevel>,
    freq: u32,
    duty: f64,
    t: u32,
    pub hz_span: Duration,
    pub hi_span: Duration,
    pub lo_span: Duration,
}
impl IrData {
    pub fn new(
        pattern: SignalPattern,
        freq: u32,
        duty: f64,
        t: u32
    ) -> Self {
        let hz_span = Duration::from_secs_f64(1.0 / freq as f64);
        let hi_span = hz_span.div_f64(duty);
        let lo_span = hz_span - hi_span;
        let bits = Self::to_bits(&pattern.signals, t);
        Self {
            name: pattern.name,
            bits,
            freq,
            duty,
            t,
            hz_span,
            hi_span,
            lo_span,
        }
    }
    pub fn to_bits(pattern: &Vec<Signal>, t: u32) -> Vec<PinLevel> {
        let mut bits = vec![];
        for p in pattern {
println!("a");
            let bits_count = p.elapsed.as_nanos() / t as u128;
println!("b");
            for _ in 0..bits_count {
                bits.push(p.level.clone());
            }
        }
        bits
    }
    pub fn play(&self, out_pin: &mut OutputPin) {
        for level in &self.bits {
            match level {
                PinLevel::High => {
                    out_pin.set_high();
                    Self::wait(self.hi_span);
                    out_pin.set_low();
                    Self::wait(self.lo_span);
                },
                PinLevel::Low => {
                    Self::wait(self.hz_span);
                }
            }
        }
    }
    fn wait(span: Duration) {
        let start = Instant::now();
println!("koba1");
        while start.elapsed() <= span {}
    }
}