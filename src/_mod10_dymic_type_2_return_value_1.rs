use std::any::Any;
fn dynamic_return(i: i8) -> Box<dyn Any> {
    match i {
        1 => return Box::new(3.14),
        2 => return Box::new("any"),
        3 => return Box::new(true),
        _ => return Box::new(-1),
    }
}

pub(crate) fn test() {

    let mut pi = dynamic_return(1);
    //pi is 3.14
    let mut s = dynamic_return(2);
    //s is "any"
    let mut b = dynamic_return(3);
    //b is True
    println!("pi is {:?}", pi.downcast_mut::<f64>());
    println!("s is {:?}", s.downcast_mut::<&str>());
    println!("b is {:?}", b.downcast_mut::<bool>());

}