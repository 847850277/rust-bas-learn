//In Rust there are no properties
trait Car {
    fn get_engine_volume(&self) -> i32;
    fn get_name(&self) -> &str;
}
struct Airwave;
impl Car for Airwave {
    fn get_engine_volume(&self) -> i32 {
        return 1500;
    }
    fn get_name(&self) -> &str {
        return "Honda Airwave";
    }
}

pub(crate) fn test() {

    let car = Airwave{};
    println!("engine_volume is {}",
             car.get_engine_volume());
    println!("name is '{}'", car.get_name());

}