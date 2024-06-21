trait Car {
    fn start_engine(&mut self) -> bool;
    fn stop_engine(&mut self);
}
struct SportCar {
    started: bool
}
impl Car for SportCar {
    fn start_engine(&mut self) -> bool {
        if self.started {
            return false;
        }
        println!("Engine started");
        self.started = true;
        return true;
    }
    fn stop_engine(&mut self) {
        self.started = false;
        println!("Engine stopped");
    }
}

pub(crate) fn test() {

    let mut car = SportCar {
        started: false };
    car.start_engine();
    car.stop_engine();

}