fn throw_panic(cars: Vec<&str>) {
    if cars.len() == 0 {
        panic!("No cars for sale");
    }
    //some implementation...
}
pub(crate) fn test() {
    throw_panic(vec![]);
}
