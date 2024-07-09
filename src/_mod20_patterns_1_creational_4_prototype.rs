use std::any::Any;
//Shape is Prototype
trait Shape {
    fn clone(&self) -> Box<dyn Shape>;
    fn as_any(&self) -> &dyn Any;
}
//Square is ConcretePrototype
#[derive(Debug)]
struct Square {
    line_count: i32
}
impl Shape for Square {
    fn clone(&self) -> Box<dyn Shape> {
        return Box::from(Square {
            line_count: self.line_count
        });
    }
    fn as_any(&self) -> &dyn Any {
        return self;
    }
}
//ShapeMaker contains a Shape
struct ShapeMaker {
    shape: Box<dyn Shape>
}
//MakeShape creates a copy of the Shape
impl ShapeMaker {
    fn make_shape(&self) -> Box<dyn Shape> {
        return Box::from(self.shape.clone());
    }
}

pub(crate) fn test() {

    //Client
    let square = Square{ line_count: 4 };
    let maker = ShapeMaker{
        shape: Box::from(square) };
    let square1 = maker.make_shape();
    let square2 = maker.make_shape();
    println!("square1 is {:?}", square1.as_any()
        .downcast_ref::<Square>().unwrap());
    println!("square2 is {:?}", square2.as_any()
        .downcast_ref::<Square>());

}