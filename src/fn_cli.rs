use crate::m_operation_type::OperationType;

pub fn get_operation_type_from_cli() -> Option<OperationType> {
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
pub fn get_profile_name_from_cli() -> Option<String> {
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return None;
    }
    Some(input.trim().to_string())
}
pub fn get_is_reverse_phase_from_cli() -> Option<bool> {
    let mut input = String::new();
    if let Err(_) = std::io::stdin().read_line(&mut input) {
        return None;
    }
    Some(input.trim().to_lowercase().as_str() == "reverse")
}