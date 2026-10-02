use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OperationType {
    Send,
    Receive,
}
impl OperationType {
    pub fn to_string(&self) -> String {
        match self {
            OperationType::Receive => "Receive".to_string(),
            OperationType::Send => "Send".to_string(),
        }
    }
    pub fn try_parse(text: &str) -> Result<Self, &str> {
        let text = text.to_lowercase();
        match text.to_lowercase().as_str() {
            "receive" => Ok(OperationType::Receive),
            "send" => Ok(OperationType::Send),
            _ => return Err("Err"),
        }
    }
}
impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = match self {
            Self::Receive => "Receive",
            Self::Send => "Send",
        };
        f.write_str(disp)
    }
}