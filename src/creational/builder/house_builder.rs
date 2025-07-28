use super::house::House;

pub trait HouseBuilder {
    fn build_foundation(self) -> Self;
    fn build_structure(self) -> Self;
    fn build_roof(self) -> Self;
    fn build_base(self) -> Self;
    fn build(self) -> House;
}
