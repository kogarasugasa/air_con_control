use rppal::gpio;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ButtonPinNumber {
    Button1 = 22,
    Button2 = 23,
    Button3 = 24,
    Button4 = 25,
    Button5 = 26,
    Button6 = 27,
}
impl ButtonPinNumber {
    pub fn get_pin(&self) -> gpio::InputPin {
        gpio::Gpio::new().unwrap()
            .get(*self as u8).unwrap()
            .into_input_pullup()
    }
}