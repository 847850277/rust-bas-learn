use std::any::Any;
pub(crate) fn test() {
    let mut d: Box<dyn Any> = Box::new("some string");
    println!("{:?}", d.downcast_mut::<&str>());
    d = Box::new(3.14);
    println!("{:?}", d.downcast_mut::<f64>());
    d = Box::new([2, 3, 5]);
    println!("{:?}", d.downcast_mut::<[i32; 3]>());
}
