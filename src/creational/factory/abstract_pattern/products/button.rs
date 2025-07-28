// PRODUCT
pub trait Button {
    fn paint(&self) -> &str;
}

// CONCRETE PRODUCT
pub struct WinButton;
impl Button for WinButton {
    fn paint(&self) -> &str {
        "Rendering a button in Windows style."
    }
}

// CONCRETE PRODUCT
pub struct MacButton;
impl Button for MacButton {
    fn paint(&self) -> &str {
        "Rendering a button in Mac style."
    }
}
