use std::ops::Neg;
struct Point {
    x: i32,
    y: i32,
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub(crate) fn test() {
    let mut p = Point { x: 3, y: 3 };
    p = -p;
    //p.x is -3, p.y is -3
    println!("x is {}, x is {}", p.x, p.y);
}
