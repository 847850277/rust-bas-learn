struct Point {
    x: i32,
    y: i32,
}
pub(crate) fn test() {
    let p1 = Point { x: 1, y: 2 };
    println!("p1 is ({}, {})", p1.x, p1.y);
}
