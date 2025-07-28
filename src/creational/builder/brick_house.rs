use super::house::House;
use super::house_builder::HouseBuilder;

// The  temporary struct
pub struct BrickHouseBuilder {
    foundation: String,
    structure: String,
    roof: String,
    base: String,
}

impl BrickHouseBuilder {
    pub fn new() -> Self {
        Self {
            foundation: String::new(),
            structure: String::new(),
            roof: String::new(),
            base: String::new(),
        }
    }
}

impl HouseBuilder for BrickHouseBuilder {
    fn build_foundation(mut self) -> Self {
        self.foundation = "Brick Foundation".to_string();
        self
    }

    fn build_structure(mut self) -> Self {
        self.structure = "Brick Structure".to_string();
        self
    }

    fn build_roof(mut self) -> Self {
        self.roof = "Brick Roof".to_string();
        self
    }

    fn build_base(mut self) -> Self {
        self.base = "Brick".to_string();
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
