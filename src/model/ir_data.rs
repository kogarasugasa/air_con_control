use std::{ops::Add, time::{Duration, Instant}};
use rppal::gpio::{OutputPin};

use crate::model::{
    signal::Signal,
    signal_pattern::SignalPattern,
    pin_level::PinLevel,
};

pub struct IrData {
    pub name: String,
    pub bits: Vec<PinLevel>,
    pub freq: f64,
    pub duty: f64,
    pub t_span: Duration,
    pub trailer_span: Duration,
    pub hz_span: Duration,
    pub hi_span: Duration,
    pub lo_span: Duration,
    pattern: SignalPattern,
}
impl IrData {
    pub fn new(
        pattern: SignalPattern,
        freq: f64,
        duty: f64,
        t_micros: u64,
        trailer_millis: u64,
    ) -> Self {
        let hz_span = Duration::from_secs_f64(1.0 / freq);
        let hi_span = hz_span.mul_f64(duty);
        let lo_span = hz_span - hi_span;
        let bits = Self::to_bits(&pattern.signals, t_micros);
        Self {
            name: pattern.name.clone(),
            bits,
            freq,
            duty,
            t_span: Duration::from_micros(t_micros as u64),
            trailer_span: Duration::from_millis(trailer_millis),
            hz_span,
            hi_span,
            lo_span,
            pattern,
        }
    }
    pub fn to_bits(pattern: &Vec<Signal>, t_micros: u64) -> Vec<PinLevel> {
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
    pub fn play_nomalize(&self, out_pin: &mut OutputPin) {
        println!("play_nomalize");
        println!("self.bits.len() is {}", self.bits.len());
        println!("hz_span is {}, hi_span is {}, lo_span is {}",
            self.hz_span.as_nanos(),
            self.hi_span.as_nanos(),
            self.lo_span.as_nanos(),
        );
        let mut cur = self.t_span.clone();
        let t_span_count = self.t_span.as_nanos() / self.hz_span.as_nanos();
        let before = Instant::now();
        for level in &self.bits {
            cur = cur.add(self.t_span);
            match level {
                PinLevel::High => {
                    for _ in 0..t_span_count {
                        out_pin.set_high();
                        Self::wait_inc(self.hi_span);
                        out_pin.set_low();
                        Self::wait_inc(self.lo_span);
                    }
                },
                PinLevel::Low => {
                    out_pin.set_low();
                    Self::wait_inc(self.t_span);
                }
            }
        }
        out_pin.set_low();
        println!("send elepsed is {:?}", before.elapsed());
        Self::wait_inc(self.trailer_span);
    }
    pub fn play_pwm(&self, out_pin: &mut OutputPin) {
        println!("play_pwm");
        println!("self.bits.len() is {}", self.bits.len());
        let freq = self.freq as f64;
        let duty = self.duty as f64;
        let mut cur = self.t_span.clone();
        let before = Instant::now();
        for level in &self.bits {
            cur = cur.add(self.t_span);
            match level {
                PinLevel::High => {
                    if let Err(e) = out_pin.set_pwm_frequency(freq, duty) {
                        println!("{e}");
                        break;
                    };
                    Self::wait_abs(cur, before);
                },
                PinLevel::Low => {
                    if let Err(e) = out_pin.clear_pwm() {
                        println!("{e}");
                        break;
                    };
                    Self::wait_abs(cur, before);
                }
            }
        }
        if let Err(e) = out_pin.clear_pwm() {
            println!("{e}");
        };
        println!("send elepsed is {:?}", before.elapsed());
        Self::wait_inc(self.trailer_span);
    }
    // pub fn play_origin(&self, out_pin: &mut OutputPin) {
    //     println!("play_origin");
    //     let before = std::time::Instant::now();
    //     for sig in &self.pattern.signals {
    //         match sig.level {
    //             PinLevel::High => {
    //                 Self::wait_abs(sig.elapsed, before);
    //                 out_pin.set_high();
    //             },
    //             PinLevel::Low => {
    //                 Self::wait_abs(sig.elapsed, before);
    //                 out_pin.set_low();
    //             }
    //         }
    //     }
    //     out_pin.set_low();
    //     println!("send elepsed is {:?}", before.elapsed());
    //     Self::wait_inc(self.trailer_span);
    // }
    fn wait_inc(span: Duration) {
        let start = Instant::now();
        while start.elapsed() <= span {}
    }
    fn wait_abs(record_elapsed: Duration, current: Instant) {
        while record_elapsed >= current.elapsed() {}
    }
}