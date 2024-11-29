//In Rust there are no abstract classes
trait Shape {
    fn fill(&self, color: &str);
    fn draw(&self) {
        println!("Draw implementation");
    }
}
struct Square {}
impl Shape for Square {
    fn fill(&self, color: &str) {
        println!("Fill implementation, {color}");
    }
}

pub(crate) fn test() {
    let square = Square {};
    square.fill("Red");
    square.draw();
}
