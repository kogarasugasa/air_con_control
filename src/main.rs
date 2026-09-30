use std::{thread, time};
use rppal::gpio::{self, Pin};

mod m_signal; use m_signal::Signal;
mod m_signal_pattern; use m_signal_pattern::SignalPattern;
mod m_operation_type; use m_operation_type::OperationType;
mod m_pin_level; use m_pin_level::PinLevel;
mod m_start_option; use m_start_option::StartOption;
mod im_signal_pattern_store;
mod fn_find_profile_names; use fn_find_profile_names::find_profile_names;
mod test; use test::sensor;

fn main() {
    sensor();
    return;
    let profile_path = std::env::current_dir();
    let profile_path = profile_path.unwrap();
    let profile_path = profile_path.to_string_lossy();
    let start_option = StartOption::new(std::env::args());
    println!("{:?}", start_option.get_operation());
    println!("{:?}", start_option.get_profile_name());

    // 操作を取得
    println!("please set operation {} or {}",
        OperationType::Receive,
        OperationType::Send
    );
    let operation = start_option.get_operation() // 引数から操作を取得する
        .or_else(|| get_operation_type_from_cli()) // 入力から操作を取得する
    ;
    let operation = match operation {
        Some(v) => v,
        None => return
    };
    // プロファイルを指定
    println!("please set profile name");
    let profile_name = start_option.get_profile_name() // 引数から操作を取得する
        .or_else(|| get_profile_name_from_cli()) // 入力から操作を取得する
    ;
    let profile_name = match profile_name {
        Some(v) => v,
        None => return
    };
    // 信号の反転を指定
    println!("please set reverse phase reverse or origin");
    let is_reverse_phase = start_option.get_is_reverse_phase()
        .or_else(|| get_is_reverse_phase_from_cli())
    ;
    // 実行
    match operation {
        OperationType::Receive => {
            println!("receive opetation");
            let pin = get_pin(4).unwrap();
            let path = create_profile_path(&profile_path, &profile_name);
            let mut pattern = SignalPattern {
                name: profile_name,
                signals: receive_pattern(pin),
            };
            if is_reverse_phase.unwrap_or(false) {
                pattern.reverse_phase();
            }
            pattern.save(&path).unwrap();
            println!("saved");
        },
        OperationType::Send => {
            println!("send opetation");
            let profile_names = find_profile_names(&profile_path).unwrap();
            let profile = profile_names.iter()
                .find(|profile| **profile == profile_name);
            match profile {
                Some(name) => {
                    let pin = get_pin(13).unwrap();
                    let path = create_profile_path(&profile_path, name);
                    let mut pattern = SignalPattern::read(&path).unwrap();
                    if is_reverse_phase.unwrap_or(false) {
                        pattern.reverse_phase();
                    }
                    send_pattern(pin, pattern);
                },
                None => {
                    println!("profile not found");
                }
            }
            println!("sended");
        }
    }
}
fn get_pin(num: u8) -> Result<gpio::Pin, gpio::Error> {
    let gpio = gpio::Gpio::new()?;
    let pin = gpio.get(num)?;
    Ok(pin)
}
fn create_profile_path(root: &str, name: &str) -> String {
    if root.ends_with("/") {
        format!("{}{}.json", root, name)
    }
    else {
        format!("{}/{}.json", root, name)
    }
}
fn send_pattern(pin: Pin, pattern: SignalPattern) {
    let mut out_pin = pin.into_output();
    for i in 0..pattern.signals.len() {
        let signal = &pattern.signals[i];
        let before = if i == 0 {
            time::Duration::ZERO
        }
        else {
            pattern.signals[i - 1].elapsed
        };
        thread::sleep(signal.elapsed - before);
        match signal.level {
            PinLevel::High => {
                out_pin.set_high();
            },
            PinLevel::Low => {
                out_pin.set_low();
            }
        }
    }
}
fn receive_pattern(pin: Pin) -> Vec<Signal> {
    let mut pattern = Vec::with_capacity(12000);
    let init_val = pin.read();
    while pin.read() == init_val {
        thread::sleep(time::Duration::from_millis(1));
    }
    let receive_timer = thread::spawn(move || thread::sleep(time::Duration::from_secs(5)));
    let std_time = time::Instant::now();
    while !receive_timer.is_finished() {
        let signal = Signal {
            level: pin.read().into(),
            elapsed: std_time.elapsed(),
        };
        pattern.push(signal);
        thread::sleep(time::Duration::from_micros(100));
    }
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
fn get_operation_type_from_cli() -> Option<OperationType> {
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        println!("{}", e);
        return None;
    };
    let operation = match OperationType::try_parse(input.trim()) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return None;
        }
    };
    Some(operation)
}
fn get_profile_name_from_cli() -> Option<String> {
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return None;
    }
    Some(input.trim().to_string())
}
fn get_is_reverse_phase_from_cli() -> Option<bool> {
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return None;
    }
    Some(input.trim().to_lowercase().as_str() == "reverse")
}
