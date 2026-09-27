use std::io::{self, Read, Write};
use std::os::windows::fs::OpenOptionsExt;
use std::{fs, thread, time};
use rppal::gpio::{ self, Pin, Level };
use serde::{Deserialize, Serialize};
use serde_json::{Serializer, Deserializer};

fn main() {
    println!("Hello, world!");
    let profile_path = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
    ;
    let mut args = std::env::args();
    let mut operation_input = String::new();
    if let Some(arg) = args.next() {
        match OperationType.parse(arg) {
            Ok(ope) => ope,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        }
        operation_input = input;
    }
    let profile_name = args.next();
    if operation_input == "" {
        if let Err(e) = io::stdin().read_line(&mut operation_input) {
            eprintln!("{}", e);
            return;
        };
    }
    let operation = match OperationType::parse(&operation_input) {
        Ok(ope) => ope,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    
    match operation {
        OperationType::Receive => {
            let mut profile_name;
            if let Err(e) = std::io::stdin().read_line(profile_name) {
                eprintln!("{}", e);
                return;
            };
            store_pattern_file(&profile_path, profile_name);
        },
        OperationType::Send => {
            let profiles = match get_profiles(&profile_path) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };
            let profile_name = match profile_name {
                Some(name) => name,
                None => return
            };
            let profile = profiles.iter()
                .find(|profile| profile == profile_name)
            ;
            if let Some(profile) = profile {
                send_pattern_file(&profile_path, &profile_name);
            }
        }
    }
}
fn get_profiles(path: &str) -> io::Result<Vec<String>> {
    let items = fs::read_dir(path)?;
    let mut profiles = vec![];
    for item in items {
        let item = item.ok()?;
        if item.file_type().ok()?.is_file() {
            profiles.push(item.file_name().to_string_lossy().into_owned());
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
    let mut file = fs::File::create(file_path)?;
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
            Level::High => {
                out_pin.set_high();
            },
            Level::Low => {
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
            level: pin.read(),
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
enum OperationType {
    Send,
    Receive,
}
impl OperationType {
    fn to_string(&self) -> String {
        match self {
            OperationType::Receive => "Receive".to_string(),
            OperationType::Send => "Send".to_string(),
        }
    }
    fn parse(text: &str) -> Result<Self, &str> {
        match text {
            "Receive" => Ok(OperationType::Receive),
            "Send" => Ok(OperationType::Send),
            _ => return Err("Err"),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Signal {
    // #[serde(with = "Level_as_string")]
    pub level: gpio::Level,
    pub elapsed: time::Duration,
}
impl Serialize for gpio::Level {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(&self.to_string())
    }
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

