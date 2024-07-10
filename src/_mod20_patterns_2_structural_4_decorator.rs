//Component
trait Shape {
    //Operation()
    fn show_info(&self);
}
//ConcreteComponent
struct Square {}
impl Shape for Square {
    fn show_info(&self) {
        print!("square");
    }
}
//Decorator
struct ShapeDecorator {
    shape: Box<dyn Shape>
}
impl Shape for ShapeDecorator {
    //Operation()
    fn show_info(&self) {
        self.shape.show_info();
    }
}
//ConcreteDecorator
struct ColorShap {
    shape: Box<dyn Shape>,
    color: String
}
impl Shape for ColorShap {
    //Operation()
    fn show_info(&self) {
        print!("{} ", self.color);
        self.shape.show_info();
    }
}
struct ShadowShape  {
    shape: Box<dyn Shape>
}
impl Shape for ShadowShape {
    //Operation()
    fn show_info(&self) {
        self.shape.show_info();
        print!(" with shadow");
    }
}

pub(crate) fn test() {

    //Client
    let square = Square {};
    square.show_info();
    //printed: square
    println!();
    let color_shape = ColorShap {
        shape: Box::new(square),
        color: String::from("red") };
    color_shape.show_info();
    //printed: red square
    println!();
    let shadow_shape = ShadowShape{
        shape: Box::new(color_shape) };
    shadow_shape.show_info();
    //printed: red square with shadow›

}