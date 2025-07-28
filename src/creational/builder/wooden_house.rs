use super::house::House;
use super::house_builder::HouseBuilder;

// The  temporary struct
pub struct WoodenHouseBuilder {
    foundation: String,
    structure: String,
    roof: String,
    base: String,
}

impl WoodenHouseBuilder {
    pub fn new() -> Self {
        Self {
            foundation: String::new(),
            structure: String::new(),
            roof: String::new(),
            base: String::new(),
        }
    }
}

impl HouseBuilder for WoodenHouseBuilder {
    fn build_foundation(mut self) -> Self {
        self.foundation = "Wooden Foundation".to_string();
        self
    }

    fn build_structure(mut self) -> Self {
        self.structure = "Wooden Structure".to_string();
        self
    }

    fn build_roof(mut self) -> Self {
        self.roof = "Wooden Roof".to_string();
        self
    }

    fn build_base(mut self) -> Self {
        self.base = "Wooden".to_string();
        self
    }

    fn build(self) -> House {
        House {
            foundation: self.foundation,
            structure: self.structure,
            roof: self.roof,
            base: self.base,
        }
    }
}
