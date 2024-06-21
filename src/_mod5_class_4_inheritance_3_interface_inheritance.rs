trait Shape {
    fn new(side_length: i32) -> Self;
    fn get_area(&self) -> i32;
}
struct Square {
    side_length: i32
}
impl Shape for Square {
    fn new(side_length: i32) -> Self {
        return Square {
            side_length: side_length
        }
    }
    fn get_area(&self) -> i32 {
        return self.side_length * self.side_length;
    }
}
pub(crate) fn test() {

    let square = Square::new(5);
    let area = square.get_area();
    //area is 25
    print!("area is {area}");

}