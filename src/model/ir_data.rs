use std::ops::{Sub, SubAssign};
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
    t_micros: u32,
    pub hz_span: Duration,
    pub hi_span: Duration,
    pub lo_span: Duration,
}
impl IrData {
    pub fn new(
        pattern: SignalPattern,
        freq: u32,
        duty: f64,
        t_micros: u32
    ) -> Self {
        let hz_span = Duration::from_secs_f64(1.0 / freq as f64);
        let hi_span = hz_span.mul_f64(duty);
        let lo_span = hz_span - hi_span;
        let bits = Self::to_bits(&pattern.signals, t_micros);
        Self {
            name: pattern.name,
            bits,
            freq,
            duty,
            t_micros,
            hz_span,
            hi_span,
            lo_span,
        }
    }
    pub fn to_bits(pattern: &Vec<Signal>, t_micros: u32) -> Vec<PinLevel> {
        let mut bits = vec![];
        for i in 0..(pattern.len() - 1) {
            let cur = &pattern[i];
            let nex = &pattern[i + 1];
            let span = nex.elapsed.as_micros() - cur.elapsed.as_micros();
            let bits_count = span / t_micros as u128;
            for _ in 0..bits_count {
                bits.push(cur.level.clone());
            }
        }
        let last = pattern.iter().last().unwrap().level.clone();
        bits.push(last);
        bits
    }
    pub fn play(&self, out_pin: &mut OutputPin) {
println!("bits.len() is {}", self.bits.len());
        for level in &self.bits {
            match level {
                PinLevel::High => {
                    out_pin.set_high();
                    std::thread::sleep(self.hz_span);
                    // out_pin.set_high();
                    // Self::wait(self.hi_span);
                    // out_pin.set_low();
                    // Self::wait(self.lo_span);
                },
                PinLevel::Low => {
                    Self::wait(self.hz_span);
                }
            }
        }
    }
    fn wait(span: Duration) {
        let start = Instant::now();
        while start.elapsed() <= span {}
    }
}