struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn distance_to(&self, p2: Point) -> f32 {
        let d1 = (self.x - p2.x).pow(2);
        let d2 = (self.y - p2.y).pow(2);
        return ((d1 + d2) as f32).sqrt();
    }
}

pub(crate) fn test() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 2, y: 3 };
    let distance = p1.distance_to(p2);
    //distance is 1.4142135
    println!("distance is {distance}");
}
