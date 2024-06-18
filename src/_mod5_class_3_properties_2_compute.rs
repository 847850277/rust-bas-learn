struct Square {
    side: i32
}
impl Square {
    // Mutable access.
    fn area(&self) -> i32 {
        return self.side * self.side;
    }
    fn set_area(&mut self, value: i32) {
        self.side = f64::sqrt(value.into()) as i32;
    }
}

pub(crate) fn test() {

    //In Rust there are no properties
    let mut square = Square { side: 2 };
    //square.area is 4
    println!("square.area is {}", square.area());
    square.set_area(9);
    //square.side is 3
    println!("square.side is {}", square.side);

}