struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn distance_to(p1: Point, p2: Point) -> f32 {
        let d1 = (p1.x - p2.x).pow(2);
        let d2 = (p1.y - p2.y).pow(2);
        return ((d1 + d2) as f32).sqrt();
    }
}

pub(crate) fn test() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 3 };
    let distance = Point::distance_to(p1, p2);
    //distance is 1.4142135
    println!("distance is {distance}");
}
