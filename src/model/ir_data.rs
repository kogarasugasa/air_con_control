use std::time::{Duration, Instant};
use rppal::gpio::{OutputPin};

use crate::model::{
    signal::Signal,
    signal_pattern::SignalPattern,
    pin_level::PinLevel,
};

pub struct IrData {
    pub name: String,
    pub bits: Vec<PinLevel>,
    freq: u32,
    duty: f64,
    t_micros: u32,
    pub hz_span: Duration,
    pub hi_span: Duration,
    pub lo_span: Duration,
    pattern: SignalPattern,
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
            name: pattern.name.clone(),
            bits,
            freq,
            duty,
            t_micros,
            hz_span,
            hi_span,
            lo_span,
            pattern,
        }
    }
    pub fn to_bits(pattern: &Vec<Signal>, t_micros: u32) -> Vec<PinLevel> {
        let mut bits = vec![];
        for i in 0..(pattern.len() - 1) {
            let cur = &pattern[i];
            let nex = &pattern[i + 1];
            let span = nex.elapsed.as_micros() - cur.elapsed.as_micros();
            let bits_count = (span as f64 / t_micros as f64).round() as u64;
            for _ in 0..bits_count {
                bits.push(cur.level.clone());
            }
        }
        let last = pattern.iter().last().unwrap().level.clone();
        bits.push(last);
        bits
    }
    pub fn play(&self, out_pin: &mut OutputPin) {
        println!("self.bits.len() is {}", self.bits.len());
        for level in &self.bits {
            match level {
                PinLevel::High => {
                    out_pin.set_high();
                    Self::wait(self.hz_span);
                    // Self::wait(self.hi_span);
                    // out_pin.set_low();
                    // Self::wait(self.lo_span);
                },
                PinLevel::Low => {
                    out_pin.set_low();
                    Self::wait(self.hz_span);
                }
            }
        }
    }
    pub fn play2(&self, out_pin: &mut OutputPin) {
        let before = std::time::Instant::now();
        for sig in &self.pattern.signals {
            match sig.level {
                PinLevel::High => {
                    out_pin.set_high();
                    Self::wait2(sig.elapsed, before);
                },
                PinLevel::Low => {
                    out_pin.set_low();
                    Self::wait2(sig.elapsed, before);
                }
            }
        }
        println!("send elepsed is {:?}", before.elapsed());
    }
    fn wait(span: Duration) {
        let start = Instant::now();
        while start.elapsed() <= span {}
    }
    fn wait2(record_elapsed: Duration, current: Instant) {
        while record_elapsed >= current.elapsed() {}
    }
}