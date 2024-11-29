struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x: x, y: y };
    }
}

pub(crate) fn test() {
    let p1 = Point::new(1, 2);
    println!("p1 is ({}, {})", p1.x, p1.y);
}
