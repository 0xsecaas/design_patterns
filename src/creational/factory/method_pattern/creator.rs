//! Declaring the factory method
use super::product::{Ship, Transport, Truck};

/// The Creator
pub trait Logistics {
    fn create_transport(&self) -> Box<dyn Transport>;
}

/// Concrete Creator A
pub struct RoadLogistics;
impl Logistics for RoadLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Truck)
    }
}

/// Concrete Creator B
pub struct SeaLogistics;
impl Logistics for SeaLogistics {
    fn create_transport(&self) -> Box<dyn Transport> {
        Box::new(Ship)
    }
}
