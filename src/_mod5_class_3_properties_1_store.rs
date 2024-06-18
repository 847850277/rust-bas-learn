//In Rust there are no properties
struct Point {
    x: i32,
    y: i32
}
pub(crate) fn test() {

    let mut point = Point { x: 0, y: 0 };
    println!("{}, {}", point.x, point.y);
    //x and y is 0
    point.x = 3;
    point.y = 7;
    println!("{}, {}", point.x, point.y);

}