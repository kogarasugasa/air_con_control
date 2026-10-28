use rppal::gpio;

use air_con_control::model::{
    // button::Button,
    // button_pin_number::ButtonPinNumber,
    start_option::StartOption,
    operation_type::OperationType,
    signal_pattern::SignalPattern,
};

use air_con_control::fn_find_profile_names::find_profile_names;
use air_con_control::fn_cli::{
    get_operation_type_from_cli,
    get_profile_name_from_cli,
};
use air_con_control::fn_create_profile_path::create_profile_path;
use air_con_control::fn_ir_tranceiver::{
    get_pin, receive_pattern, send_pattern, send_pattern_pwm
};

fn main() {
    let profile_path = std::env::current_dir();
    let profile_path = profile_path.unwrap();
    let profile_path = profile_path.to_string_lossy();
    let profile_path = format!("{}/profiles", profile_path);
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
    // 実行
    ir_tranceiver(operation, profile_name.clone(), &profile_path);
    // return;
    // let ope_send_btn = Button {
    //     pin: ButtonPinNumber::Button1.get_pin(),
    //     on_level: gpio::Level::Low,
    // };
    // let ope_recv_btn = Button {
    //     pin: ButtonPinNumber::Button2.get_pin(),
    //     on_level: gpio::Level::Low,
    // };
    // let reverse_btn = Button {
    //     pin: ButtonPinNumber::Button3.get_pin(),
    //     on_level: gpio::Level::Low,
    // };
    // let pro_btns = [
    //     Button {
    //         pin: ButtonPinNumber::Button4.get_pin(),
    //         on_level: gpio::Level::Low,
    //     },
    //     Button {
    //         pin: ButtonPinNumber::Button5.get_pin(),
    //         on_level: gpio::Level::Low,
    //     },
    // ];
    // let exe_btn = Button {
    //     pin: ButtonPinNumber::Button6.get_pin(),
    //     on_level: gpio::Level::Low,
    // };
    // let mut operation = None;
    // let mut profile_name = None;
    // let mut is_reverse_phase = None;
    // loop {
    //     if operation.is_none() {
    //         if ope_recv_btn.is_on() {
    //             operation = Some(OperationType::Receive);
    //             println!("receive operation selected");
    //         }
    //         else if ope_send_btn.is_on() {
    //             operation = Some(OperationType::Send);
    //             println!("send operation selected");
    //         }
    //     }
    //     if is_reverse_phase.is_none() {
    //         if reverse_btn.is_on() {
    //             is_reverse_phase = Some(true);
    //             println!("reverse phase selected");
    //         }
    //     }
    //     if profile_name.is_none() {
    //         for (i, btn) in pro_btns.iter().enumerate() {
    //             if btn.is_on() {
    //                 profile_name = Some(format!("profile{}", i + 1));
    //                 println!("profile{} selected", i + 1);
    //                 break;
    //             }
    //         }
    //     }
    //     if exe_btn.is_on() {
    //         let operation = match operation.take() {
    //             Some(v) => v,
    //             None => continue
    //         };
    //         let profile_name = match profile_name.take() {
    //             Some(v) => v,
    //             None => continue
    //         };
    //         let is_reverse_phase = match is_reverse_phase.take() {
    //             Some(v) => v,
    //             None => false
    //         };
    //         println!("execute operation: {} / profile: {} / reverse phase: {}",
    //             operation,
    //             profile_name,
    //             is_reverse_phase
    //         );
    //         ir_tranceiver(operation, profile_name, &profile_path);
    //     }
    //     std::thread::sleep(std::time::Duration::from_millis(10));
    // }
}
fn ir_tranceiver(
    operation: OperationType,
    profile_name: String,
    root: &str
) {
    println!(
        "execute operation: {operation} / profile: {profile_name}"
    );
    match operation {
        OperationType::Receive => {
            println!("receive opetation");
            let pattern = SignalPattern {
                name: profile_name.to_string(),
                signals: receive_pattern(get_pin(4).unwrap()),
            };
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
                    let pattern = match SignalPattern::read(&path) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    };
                    send_pattern(get_pin(13).unwrap(), pattern.clone());
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    send_pattern_pwm(pattern);
                },
                None => {
                    println!("profile not found");
                }
            }
            println!("sended");
        }
    }
}