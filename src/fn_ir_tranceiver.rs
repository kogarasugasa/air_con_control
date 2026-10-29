use std::{thread, time};
use rppal::gpio::{self,};

use crate::model::{
    signal::Signal,
    signal_pattern::SignalPattern,
    ir_data::IrData,
};

pub fn send_pattern(pin: gpio::Pin, pattern: SignalPattern) {
    let freq = 38000.0; // 38kHz
    let duty = 1.0 / 3.0;
    let t_micros = 420;
    let trailer_millis = 8;
    let ir_data = IrData::new(pattern, freq, duty, t_micros, trailer_millis);
    let mut out_pin = pin.into_output();
    ir_data.play_nomalize(&mut out_pin);
    thread::sleep(time::Duration::from_millis(130));
    out_pin.set_low();
    println!("send_pattern() end")
}
pub fn send_pattern_pwm(pattern: SignalPattern) {
    let freq = 38000.0; // 38kHz
    let duty = 1.0 / 3.0;
    let t_micros = 425;
    let trailer_millis = 8;
    let ir_data = IrData::new(pattern, freq, duty, t_micros, trailer_millis);
    ir_data.play_pwm();
    thread::sleep(time::Duration::from_millis(130));
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
