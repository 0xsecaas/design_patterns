//! Define the common behavior
/// The Product
pub trait Transport {
    fn deliver(&self) -> bool;
}

/// Concrete Product A
pub struct Truck;
impl Transport for Truck {
    fn deliver(&self) -> bool {
        println!("Delivering ]by land in a box.");
        true
    }
}

/// Concrete Product B
pub struct Ship;
impl Transport for Ship {
    fn deliver(&self) -> bool {
        print!("Deliverin by sea in a container");
        true
    }
}
