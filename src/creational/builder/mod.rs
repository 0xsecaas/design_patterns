//! Separates the construction from representaion.
//!
//!
//! Enables construction of complex objects step by step, in a modular pattern.
//!
//! By embracing Rust's ownership model and method chaining, this pattern enhances code readability
//! and maintainability.

mod brick_house;
mod house;
mod house_builder;
mod wooden_house;

#[cfg(test)]
mod tests {
    use super::brick_house::BrickHouseBuilder;
    use super::house::House;
    use super::house_builder::HouseBuilder;
    use super::wooden_house::WoodenHouseBuilder;

    #[test]
    fn build_wooden_house() {
        let house: House = WoodenHouseBuilder::new()
            .build_base()
            .build_foundation()
            .build_structure()
            .build_roof()
            .build();
        house.show();
        assert_eq!(house.base(), "Wooden");
    }

    #[test]
    fn build_brick_house() {
        let house: House = BrickHouseBuilder::new()
            .build_base()
            .build_foundation()
            .build_structure()
            .build_roof()
            .build();
        house.show();
        assert_eq!(house.base(), "Brick");
    }
}
