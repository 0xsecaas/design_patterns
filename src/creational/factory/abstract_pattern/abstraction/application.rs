use super::creator::factory::GUIFactory;
use super::products::{button, checkbox};
use button::Button;
use checkbox::Checkbox;

pub struct Application {
    button: Box<dyn Button>,
    checkbox: Box<dyn Checkbox>,
}
impl Application {
    pub fn new(factory: Box<dyn GUIFactory>) -> Self {
        Self {
            button: factory.create_button(),
            checkbox: factory.create_checkbox(),
        }
    }

    pub fn paint(&self) -> Vec<&str> {
        vec![self.button.paint(), self.checkbox.paint()]
    }
}
