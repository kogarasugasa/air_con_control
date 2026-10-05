use std::{thread, time};
use rppal::gpio::{self,};

use crate::m_signal::Signal;
use crate::m_signal_pattern::SignalPattern;
use crate::m_pin_level::PinLevel;

pub fn send_pattern(pin: gpio::Pin, pattern: SignalPattern) {
    let freq = 38000.0; // 38kHz
    let duty = 1.0 / 3.0;
    let hz_span = time::Duration::from_secs_f64(1.0 / freq); // 1Hzの時間
    let high_span = hz_span.mul_f64(duty);
    let low_span = hz_span - high_span;
    
    let mut out_pin = pin.into_output();
    let first = match pattern.signals.first() {
        Some(v) => v,
        None => return
    };
    // ピンの初期状態を設定する
    match first.level {
        PinLevel::High => out_pin.set_high(),
        PinLevel::Low => out_pin.set_low()
    }
    // 信号のパターンを出力する
    let last_signal = match pattern.signals.last() {
        Some(v) => v,
        None => return
    };
    let range = 0..pattern.signals.len() - 1;
    for i in range {
        let signal = &pattern.signals[i];
        let next = pattern.signals[i + 1].elapsed;
        let span = next - signal.elapsed;

        match signal.level {
            PinLevel::High => {
                let mut total_span = time::Duration::ZERO;
                while span > total_span {
                    out_pin.set_high();
                    //thread::sleep(high_span);
                    wait(high_span);
                    out_pin.set_low();
                    //thread::sleep(low_span);
                    wait(low_span);
                    total_span += hz_span;
                }
            },
            //PinLevel::Low => thread::sleep(span)
            PinLevel::Low => wait(span)
        }
    }
    match last_signal.level {
        PinLevel::High => out_pin.set_high(),
        PinLevel::Low => out_pin.set_low(),
    }
    thread::sleep(time::Duration::from_millis(130));
    out_pin.set_low();
    println!("send_pattern() end")
}
pub fn receive_pattern(pin: gpio::Pin) -> Vec<Signal> {
    let freq = 38000.0; // 38kHz
    let hz_span = time::Duration::from_secs_f64(1.0 / freq); // 1Hzの時間
    let recording_span = time::Duration::from_secs(4); // 4秒
    let input_pin = pin.into_input_pulldown();
    // 事前に容量を確保しておくことで、パターンの記録中にメモリの再確保が発生するのを防ぐ
    let pre_capacity = recording_span.div_duration_f64(hz_span) as usize;
    let mut pattern = Vec::with_capacity(pre_capacity);
    let init_val = input_pin.read();
    while input_pin.read() == init_val {
        thread::sleep(time::Duration::from_micros(1));
    }
    let receive_timer = thread::spawn(move || thread::sleep(recording_span));
    let std_time = time::Instant::now();
    while !receive_timer.is_finished() {
        let signal = Signal {
            level: input_pin.read().into(),
            elapsed: std_time.elapsed(),
        };
        pattern.push(signal);
        //thread::sleep(time::Duration::from_micros(span_micros));
    }

    // 信号の変化のない部分を削除する
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
pub fn get_pin(num: u8) -> Result<gpio::Pin, gpio::Error> {
    let pin = gpio::Gpio::new()?.get(num)?;
    Ok(pin)
}
pub fn wait(span: time::Duration) {
    let start = time::Instant::now();
    while (time::Instant::now() - start) <= span {}
}