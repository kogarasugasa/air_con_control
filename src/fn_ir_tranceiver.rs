use std::ops::AddAssign;
use std::{thread, time};
use rppal::gpio::{self,};

use crate::m_signal::Signal;
use crate::m_signal_pattern::SignalPattern;
use crate::m_pin_level::PinLevel;
use crate::model::IrData;

pub fn send_pattern(pin: gpio::Pin, pattern: SignalPattern) {
    //let pattern = normalize(pattern);
    let freq = 38000; // 38kHz
    let duty = 1.0 / 3.0;
    let t_micros = 425;
    let ir_data = IrData::new(pattern, freq, duty, t_micros);
    let mut out_pin = pin.into_output();
    ir_data.play(&mut out_pin);
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
// url https://elm-chan.org/docs/ir_format.html
fn normalize(pattern: SignalPattern) -> SignalPattern {
    let mut nomalized = vec![];
    let mut format = None;
    let mut total = time::Duration::ZERO;
    for i in 0..(pattern.signals.len() -1) {
        let cur = &pattern.signals[i];
        let span = pattern.signals[i + 1].elapsed - cur.elapsed;
        if 8992 - 100 <= span.as_micros()
        && span.as_micros() <= 8992 + 100 {
            if format.is_none() {
                format = Some(SignalFormat::NEC);
            }
        }
        else if 2400 - 100 <= span.as_micros()
        && span.as_micros() <= 2400 + 100 {
            if format.is_none() {
                format = Some(SignalFormat::AEHA(38000, 425));
            }
        }
        else if 2800 - 100 <= span.as_micros()
        && span.as_micros() <= 4000 + 100 {
            if format.is_none() {
                format = Some(SignalFormat::SONY);
            }
        }
        let format = match format.take() {
            Some(v) => v,
            None => {
                total.add_assign(span);
                let elapsed = total.clone();
                let signal = Signal { level: cur.level.clone(), elapsed };
                nomalized.push(signal);
                continue;
            }
        };

        match format {
            SignalFormat::NEC => {
                let signal_count = (span.as_micros() as f64 / 562.0).round();
                let nomalized_span = time::Duration::from_micros(562)
                    .mul_f64(signal_count);
                total.add_assign(nomalized_span);
                let elapsed = total.clone();
                let signal = Signal { level: cur.level.clone(), elapsed: elapsed };
                nomalized.push(signal);
            },
            SignalFormat::AEHA(_, _) => {
                let signal_count = (span.as_micros() as f64 / 425.0).round();
                let nomalized_span = time::Duration::from_micros(425)
                    .mul_f64(signal_count);
                total.add_assign(nomalized_span);
                let elapsed = total.clone();
                let signal = Signal { level: cur.level.clone(), elapsed: elapsed };
                nomalized.push(signal);
            },
            SignalFormat::SONY => {
                let signal_count = (span.as_micros() as f64 / 600.0).round();
                let nomalized_span = time::Duration::from_micros(600)
                    .mul_f64(signal_count);
                total.add_assign(nomalized_span);
                let elapsed = total.clone();
                let signal = Signal { level: cur.level.clone(), elapsed: elapsed };
                nomalized.push(signal);
            }
        }
    }
    SignalPattern { name: pattern.name, signals: nomalized }

}
enum SignalFormat {
    NEC,
    AEHA(u32, u32),
    SONY,
}

