// PRODUCT
pub trait Checkbox {
    fn paint(&self) -> &str;
}

pub struct WinCheckbox;
impl Checkbox for WinCheckbox {
    fn paint(&self) -> &str {
        "Rendering a checkbox in Windows style."
    }
}

pub struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn paint(&self) -> &str {
        "Rendering a checkbox in Mac style."
    }
}
