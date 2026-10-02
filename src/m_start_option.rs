use std::env;
use crate::m_operation_type::OperationType;

pub struct StartOption {
    pub args: Vec<String>,
}
impl StartOption {
    pub fn new(args: env::Args) -> Self {
        let args: Vec<String> = args.into_iter()
            .map(|v| v.to_lowercase())
            .collect()
        ;
        Self { args }
    }
    pub fn is_some(&self) -> bool {
        self.args.len() >= 1
    }
    pub fn get_operation(&self) -> Option<OperationType> {
        let arg1 = match self.args.get(1) {
            Some(v) => v.as_str(),
            None => return None,
        };
        let operation = match OperationType::try_parse(arg1) {
            Ok(v) => v,
            Err(_) => return None,
        };
        Some(operation)
    }
    pub fn get_profile_name(&self) -> Option<String> {
        match self.args.get(2) {
            Some(v) => Some(v.to_string()),
            None => None,
        }
    }
    pub fn get_is_reverse_phase(&self) -> Option<bool> {
        let text = match self.args.get(3) {
            Some(v) => v,
            None => return None,
        };
        Some(text.to_lowercase().as_str() == "reverse")
    }
}