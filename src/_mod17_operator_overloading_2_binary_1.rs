use std::ops::{Add, AddAssign};
struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

pub(crate) fn test() {

    let p1 = Point{ x: 1, y: 1 };
    let p2 = Point{ x: 2, y: 2 };
    let mut p3 = p1 + p2;
    //p3.x is 3 and p3.y is 3
    println!("x is {}, x is {}", p3.x, p3.y);
    p3 += Point{ x: 3, y: 5 };
    //p3.x is 6 and p3.y is 8
    println!("x is {}, x is {}", p3.x, p3.y);


}