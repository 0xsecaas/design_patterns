//!
//! Support multiple platforms or configuration enabling Scalable applications
//! Create a group of objects without specifying their concrete classes.

mod abstraction;
mod creator;
mod products;

#[cfg(test)]
mod tests {
    use super::*;
    use creator::factory::{GUIFactory, MacFactory, WinFactory};

    #[test]
    fn test_abstract_pattern_win_style() {
        let app: Box<dyn GUIFactory> = Box::new(WinFactory);
        let button = app.create_button();
        assert_eq!(button.paint(), "Rendering a button in Windows style.");

        let checkbox = app.create_checkbox();
        assert_eq!(checkbox.paint(), "Rendering a checkbox in Windows style.");
    }

    #[test]
    fn test_abstract_pattern_mac_style() {
        let app: Box<dyn GUIFactory> = Box::new(MacFactory);
        let button = app.create_button();
        assert_eq!(button.paint(), "Rendering a button in Mac style.");

        let checkbox = app.create_checkbox();
        assert_eq!(checkbox.paint(), "Rendering a checkbox in Mac style.");
    }
}
