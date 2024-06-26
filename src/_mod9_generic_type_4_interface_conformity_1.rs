trait Vehicle {
    fn test(&self);
}
struct Car;
#[allow(dead_code)]
struct Truck;
impl Vehicle for Car {
    fn test(&self) {
        println!("Test car");
    }
}
struct Service<T> where T: Vehicle {
    list: Vec<T>
}
impl<T: Vehicle> Service<T> {
    fn add(&mut self, item: T) {
        self.list.push(item);
    }
    fn test(&self) {
        for item in &self.list {
            item.test();
        }
    }
}

pub(crate) fn test() {

    let mut service = Service{list: vec![]};
    service.add(Car{});
    service.test()
    //service.add(Truck{}); //<-Error

}