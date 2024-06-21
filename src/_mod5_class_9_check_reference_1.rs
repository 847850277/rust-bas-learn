//There are no classes in Rust
#[derive(Copy, Clone)]
struct Car {
    speed: i32
}
impl PartialEq for Car {
    fn eq(&self, other: &Self) -> bool {
        self.speed == other.speed
    }
}

pub(crate) fn test() {

    let car1 = Car { speed: 50 };
    let car2 = Car { speed: 90 };
    let car3 = car1;
    let equal1 = car1 == car2;
    //equal1 is false
    let equal2 = car1 == car3;
    //equal2 is true
    println!("equal1 is {equal1}");
    println!("equal2 is {equal2}");

}

