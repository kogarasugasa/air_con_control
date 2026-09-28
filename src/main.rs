use std::env::Args;
use std::fmt::Display;
use std::io::{self, Read, Write};
use std::{fs, thread, time};
use rppal::gpio::{ self, Pin, Level };
use serde::{Deserialize, Serialize};


fn main() {
    let profile_path = std::env::current_dir();
    let profile_path = profile_path.unwrap();
    let profile_path = profile_path.to_string_lossy();
    let args: Vec<String> = std::env::args().into_iter()
        .map(|v| v.to_lowercase())
        .collect()
    ;
    let start_option = StartOption { args };

    // 操作を取得
    println!("please set operation {} or {}",
        OperationType::Receive,
        OperationType::Send
    );
    let operation = start_option.get_operation() // 引数から操作を取得する
        .or(get_operation_type_from_cli()) // 入力から操作を取得する
    ;
    let operation = match operation {
        Some(v) => v,
        None => return
    };

    // プロファイルを指定
    println!("please set profile name");
    let profile_name = start_option.get_profile_name() // 引数から操作を取得する
        .or(get_profile_name_from_cli()) // 入力から操作を取得する
    ;
    let profile_name = match profile_name {
        Some(v) => v,
        None => return
    };
    match operation {
        OperationType::Receive => {
            store_pattern_file(&profile_path, &profile_name);
        },
        OperationType::Send => {
            let profiles = match get_profiles(&profile_path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
            let profile = profiles.iter()
                .find(|profile| **profile == profile_name)
            ;
            if let Some(profile) = profile {
                send_pattern_file(&profile_path, &profile);
            }
        }
    }
}
fn get_profiles(path: &str) -> io::Result<Vec<String>> {
    let items = fs::read_dir(path)?;
    let mut profiles = vec![];
    for item in items {
        let item = match item.ok() {
            Some(v) => v,
            None => continue
        };
        let item_type = match item.file_type().ok() {
            Some(v) => v,
            None => continue
        };
        let name = item.file_name().to_string_lossy().into_owned();
        if item_type.is_file() {
            profiles.push(name);
        }
    }
    Ok(profiles)
}
fn send_pattern_file(root: &str, name: &str) {
    let pin = match gpio::Gpio::new() {
        Ok(gpio) => gpio.get(13),
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
    let pattern = match read_pattern_from_file(name) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        },
    };
    send_pattern(pin, pattern);
}
fn store_pattern_file(root: &str, name: &str) {
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
    let pattern = SignalPattern {
        name: name.to_string(),
        signals: receive_pattern(pin),
    };
    let path = pattern.name.clone() + ".json";
    let is_exists = match fs::exists(&path) {
        Ok(exists) => exists,
        Err(e) =>  false
    };
    if is_exists {
        match fs::remove_file(&path) {
            Ok(_) => println!("Previous pattern file removed"),
            Err(e) => eprintln!("Failed to remove previous pattern file: {}", e),
        }
    }
    match save_pattern_to_file(&pattern, &path) {
        Ok(_) => println!("Pattern saved to signal_pattern.txt"),
        Err(e) => eprintln!("Failed to save pattern: {}", e),
    }
}

fn read_pattern_from_file(file_path: &str) -> Result<SignalPattern, String> {
    let mut file = match fs::File::options()
        .read(true)
        .open(file_path) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };
    let mut json = Default::default();
    if let Err(e) = file.read_to_string(&mut json) {
        return Err(e.to_string());
    }
    let pattern: SignalPattern = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };
    Ok(pattern)
}
fn save_pattern_to_file(pattern: &SignalPattern, file_path: &str) -> Result<(), String> {
    let mut file = match fs::File::create(file_path) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };
    let json = match serde_json::to_string(&pattern) {
        Ok(json) => json,
        Err(e) => return Err(e.to_string())
    };
    if let Err(e) = file.write(json.as_bytes()) {
        return Err(e.to_string());
    };
    Ok(())
}

fn send_pattern(pin: Pin, pattern: SignalPattern) {
    let mut out_pin = pin.into_output();
    for i in 0..pattern.signals.len() {
        let signal = &pattern.signals[i];
        let before = match pattern.signals.get(i -1) {
            Some(pat) => pat.elapsed,
            None => time::Duration::ZERO
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
    let mut pattern = vec![];
    let init_val = pin.read();
    while true {
        let val = pin.read();
        if val != init_val {
            break;
        }
        thread::sleep(time::Duration::from_millis(1));
    }
    let receive_timer = thread::spawn(move || thread::sleep(time::Duration::from_secs(5)));
    let std_time = time::Instant::now();
    while receive_timer.is_finished() == false {
        let signal = Signal {
            level: pin.read().into(),
            elapsed: std_time.elapsed(),
        };
        pattern.push(signal);
        thread::sleep(time::Duration::from_millis(1));
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
#[derive(Debug, PartialEq, Eq)]
enum OperationType {
    Send,
    Receive,
}
impl Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp = match self {
            Self::Receive => "Receive",
            Self::Send => "Send",
        };
        f.write_str(disp)
    }
}
impl OperationType {
    fn to_string(&self) -> String {
        match self {
            OperationType::Receive => "Receive".to_string(),
            OperationType::Send => "Send".to_string(),
        }
    }
    fn try_parse(text: &str) -> Result<Self, &str> {
        match text.to_lowercase().as_str() {
            "receive" => Ok(OperationType::Receive),
            "send" => Ok(OperationType::Send),
            _ => return Err("Err"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
enum PinLevel {
    High,
    Low
}
impl From<gpio::Level> for PinLevel {
    fn from(value: gpio::Level) -> Self {
        match value {
            gpio::Level::High => Self::High,
            gpio::Level::Low => Self::Low,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Signal {
    pub level: PinLevel,
    pub elapsed: time::Duration,
}

#[derive(Serialize, Deserialize)]
pub struct SignalPattern {
    pub name: String,
    pub signals: Vec<Signal>,
}

impl SignalPattern {
    pub fn duration(&self) -> std::time::Duration {
        self.signals.iter().map(|sig| sig.elapsed).sum()
    }
}

struct StartOption {
    args: Vec<String>,
}
impl StartOption {
    fn get_operation(&self) -> Option<OperationType> {
        let arg1 = match self.args.get(0) {
            Some(v) => v.as_str(),
            None => return None,
        };
        let operation = match OperationType::try_parse(arg1) {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(operation)
    }
    fn get_profile_name(&self) -> Option<String> {
        match self.args.get(1) {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
}
fn get_operation_type_from_cli() -> Option<OperationType> {
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        println!("{}", e);
        return None;
    };
    let operation = match OperationType::try_parse(&input) {
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
    Some(input)
}