struct Size {
    width: i32,
    height: i32
}
struct Point {
    x: i32,
    y: i32
}
struct Rect {
    size: Size,
    point: Point
}

pub(crate) fn test() {
    let rect = Rect {
        size: Size {width: 10, height: 12},
        point: Point {x: 3, y: 4}
    };

    println!("size is ({}, {})",
             rect.size.width, rect.size.height);
    println!("point is ({}, {})",
             rect.point.x, rect.point.y);

}