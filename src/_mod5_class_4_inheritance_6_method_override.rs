trait Shape {
    fn get_area(&self) -> i32 {
        return 0;
    }
}
struct Square {
    side_length: i32,
}
impl Shape for Square {
    fn get_area(&self) -> i32 {
        return self.side_length * self.side_length;
    }
}

pub(crate) fn test() {
    let square = Square { side_length: 5 };
    let area = square.get_area();
    //area is 25
    print!("area is {area}");
}
