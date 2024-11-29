trait Vehicle {
    fn test(&self) {
        println!("Test {}", std::any::type_name::<Self>())
    }
}
struct Car;
struct Truck;
impl Vehicle for Car {}
impl Vehicle for Truck {}

pub(crate) fn test() {
    let rows: [Box<dyn Vehicle>; 2] = [Box::new(Car {}), Box::new(Truck {})];
    for vehicle in rows {
        vehicle.test();
    }
}
