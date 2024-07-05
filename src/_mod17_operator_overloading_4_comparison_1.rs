#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
use std::cmp::Ordering;
impl PartialOrd<Point> for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if (self.x > other.x) && (self.y > other.y) {
            return Some(Ordering::Greater);
        } else {
            return Some(Ordering::Less);
        }
    }
}

pub(crate) fn test() {

    let p1 = Point{ x: 1, y: 2 };
    let p2 = Point{ x: 2, y: 3 };
    let b1 = p1 > p2;
    //b1 is false
    let b2 = p1 < p2;
    //b2 is true
    println!("b1 is {b1}");
    println!("b2 is {b2}");

}