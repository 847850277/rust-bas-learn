trait Shape {
    fn test(&self) {
        println!("test");
    }
}
struct Square;
struct RedSquare;
impl Shape for Square {

}

impl Shape for RedSquare {

}

pub(crate) fn test() {

    let square = Square;
    let red_square = RedSquare;
    let shape = &square as &dyn Shape;
    shape.test();
    // let red_shape = &red_square as &dyn Shape; //<-Error
    // red_shape.test();

    let red_shape = &red_square as &dyn Shape; // No error now
    red_shape.test();

}