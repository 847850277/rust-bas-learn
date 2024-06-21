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

fn test_1<T: Shape>(s: &T) {
    s.test();
}

pub(crate) fn test() {

    let square = Square;
    let red_square = RedSquare;
    test_1(&square);
    test_1(&red_square);


}