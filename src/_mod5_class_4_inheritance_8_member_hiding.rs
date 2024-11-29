//In Rust there are no abstract classes
trait Shape {
    fn fill(&self, color: &str) {
        println!("Fill implementation, {color}");
    }
}
struct Square {}
impl Shape for Square {
    fn fill(&self, color: &str) {
        println!("New fill implementation, {color}");
    }
}

pub(crate) fn test() {
    let square = Square {};
    square.fill("Red");
    //Use Square fill implementation
    Shape::fill(&square, "Red");
    //Use Square fill implementation
    let shape = &square as &dyn Shape;
    shape.fill("Red");
    //Use Square fill implementation
}
