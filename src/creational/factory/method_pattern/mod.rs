//!
//! It encapsulate object creation, allowing different implementations to alter the type of objects that will be created.
mod creator;
mod product;

#[cfg(test)]
mod tests {
    use super::creator::{Logistics, RoadLogistics, SeaLogistics};
    use super::product::Transport;

    #[test]
    fn create_truck() {
        let creator: Box<dyn Logistics> = Box::new(RoadLogistics);
        let truck: Box<dyn Transport> = creator.create_transport();
        assert!(truck.deliver());
    }

    #[test]
    fn create_ship() {
        let creator: Box<dyn Logistics> = Box::new(SeaLogistics);
        let ship: Box<dyn Transport> = creator.create_transport();
        assert!(ship.deliver());
    }
}
