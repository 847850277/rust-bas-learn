use std::ops::{BitXor};
struct Point {
    x: i32,
    y: i32,
}
impl BitXor<u32> for Point {
    type Output = Self;
    // rhs is the "right-hand side" of the expression `a ^ b`
    fn bitxor(self, rhs: u32) -> Self {
        Self {x: self.x.pow(rhs), y: self.y.pow(rhs)}
    }
}
pub(crate) fn test() {

    let mut p1 = Point{ x: 2, y: 3 };
    p1 = p1 ^ 3;
    //p1.x is 8 and p1.y is 27
    println!("x is {}, x is {}", p1.x, p1.y);

}