use rppal::gpio;

mod m_signal;
mod m_signal_pattern; use m_signal_pattern::SignalPattern;
mod m_operation_type; use m_operation_type::OperationType;
mod m_pin_level;
mod m_start_option; use m_start_option::StartOption;
mod im_signal_pattern_store;
mod fn_find_profile_names; use fn_find_profile_names::find_profile_names;
mod temperature; use serde_json::to_string;
use temperature::sensor;
mod m_button; use m_button::{Button, ButtonPinNumber};
mod fn_cli; use fn_cli::{
    get_operation_type_from_cli, get_profile_name_from_cli,
    get_is_reverse_phase_from_cli,
};
mod fn_create_profile_path; use fn_create_profile_path::create_profile_path;
mod fn_ir_tranceiver; use fn_ir_tranceiver::{
    send_pattern, receive_pattern, get_pin
};
//mod lcd; use lcd::display;

fn main() {
    sensor();
    //display();
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
    if start_option.is_some() {
        ir_tranceiver(
            operation,
            is_reverse_phase.unwrap_or(false),
            profile_name.clone(),
            &profile_path
        );
        return;
    }
    let ope_send_btn = Button {
        pin: ButtonPinNumber::Button1.get_pin(),
        on_level: gpio::Level::High,
    };
    let ope_recv_btn = Button {
        pin: ButtonPinNumber::Button2.get_pin(),
        on_level: gpio::Level::High,
    };
    let pro_btns = [
        Button {
            pin: ButtonPinNumber::Button3.get_pin(),
            on_level: gpio::Level::High,
        },
        Button {
            pin: ButtonPinNumber::Button4.get_pin(),
            on_level: gpio::Level::High,
        },
        Button {
            pin: ButtonPinNumber::Button5.get_pin(),
            on_level: gpio::Level::High,
        },
    ];
    let exe_btn = Button {
        pin: ButtonPinNumber::Button6.get_pin(),
        on_level: gpio::Level::High,
    };
    let mut operation = None;
    let mut profile_name = None;
    loop {
        if operation.is_none() {
println!("------------ 1");
            if ope_recv_btn.is_on() {
                operation = Some(OperationType::Receive);
                println!("receive operation selected");
            }
            else if ope_send_btn.is_on() {
                operation = Some(OperationType::Send);
                println!("send operation selected");
            }
        }
println!("------------ 2");
        if profile_name.is_none() {
println!("------------ 3");
            for (i, btn) in pro_btns.iter().enumerate() {
                if btn.is_on() {
                    profile_name = Some(format!("profile{}", i + 1));
                    println!("profile{} selected", i + 1);
                    break;
                }
            }
        }
        if exe_btn.is_on() {
println!("------------ 4");
            let operation = match operation.take() {
                Some(v) => v,
                None => continue
            };
println!("------------ 5");
            let profile_name = match profile_name.take() {
                Some(v) => v,
                None => continue
            };
            print!("execute operation: {} / profile: {} ... ",
                operation,
                profile_name
            );
            ir_tranceiver(
                operation,
                false,
                profile_name,
                &profile_path
            );
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
fn ir_tranceiver(
    operation: OperationType,
    is_reverse_phase: bool,
    profile_name: String,
    root: &str
) {
    match operation {
        OperationType::Receive => {
            println!("receive opetation");
            let mut pattern = SignalPattern {
                name: profile_name.to_string(),
                signals: receive_pattern(get_pin(4).unwrap()),
            };
            if is_reverse_phase {
                pattern.reverse_phase();
            }
            let path = create_profile_path(root, &profile_name);
            pattern.save(&path).unwrap();
            println!("saved");
        },
        OperationType::Send => {
            println!("send opetation");
            let profile_names = find_profile_names(root).unwrap();
            let profile = profile_names.iter()
                .find(|profile| **profile == profile_name);
            match profile {
                Some(name) => {
                    let path = create_profile_path(root, name);
                    let mut pattern = SignalPattern::read(&path).unwrap();
                    if is_reverse_phase {
                        pattern.reverse_phase();
                    }
                    send_pattern(get_pin(13).unwrap(), pattern);
                },
                None => {
                    println!("profile not found");
                }
            }
            println!("sended");
        }
    }
}