struct Point {
    x: i32,
    y: i32,
}

pub(crate) fn test() {
    let get_distance = |p1: Point, p2: Point| -> f32 {
        let d1 = (p1.x - p2.x).pow(2);
        let d2 = (p1.y - p2.y).pow(2);
        return ((d1 + d2) as f32).sqrt();
    };
    let point1 = Point { x: 0, y: 0 };
    let point2 = Point { x: 5, y: 5 };
    let distance = get_distance(point1, point2);
    //distance is 7.071068
    println!("distance is {distance}");
}
