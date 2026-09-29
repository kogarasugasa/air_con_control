use crate::m_operation_type::OperationType;

pub struct StartOption {
    pub args: Vec<String>,
}
impl StartOption {
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
}