/// The Product
trait Transport {
    fn deliver(&self) -> bool;
}

/// Concrete Product A
struct Truck;
impl Transport for Truck {
    fn deliver(&self) -> bool {
        println!("Delivering ]by land in a box.");
        return true;
    }
}

/// Concrete Product B
struct Ship;
impl Transport for Ship {
    fn deliver(&self) -> bool {
        print!("Deliverin by sea in a container");
        return true;
    }
}

/// The Creator
trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;
}

// Concrete Creator A
struct RoadLogistics;
impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck)
    }
}

/// Concrete Creator B
struct SeaLogistics;
impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship)
    }
}

#[cfg(test)]
mod tests {
    use crate::creational::factory::method::{Logistics, RoadLogistics, SeaLogistics, Transport};

    #[test]
    fn create_truck() {
        let creator: Box<dyn Logistics> = Box::new(RoadLogistics);
        let truck: Box<dyn Transport> = creator.create_transport();
        assert_eq!(truck.deliver(), true);
    }

    #[test]
    fn create_ship() {
        let creator: Box<dyn Logistics> = Box::new(SeaLogistics);
        let ship: Box<dyn Transport> = creator.create_transport();
        assert_eq!(ship.deliver(), true);
    }
}
