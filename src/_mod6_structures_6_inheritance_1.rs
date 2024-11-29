trait Text {
    fn to_text(&self) -> String;
}
struct Point {
    x: i32,
    y: i32,
}
impl Text for Point {
    fn to_text(&self) -> String {
        return format!("x = {}; y = {}", self.x, self.y);
    }
}
pub(crate) fn test() {
    let p = Point { x: 1, y: 2 };
    println!("p is '{}'", p.to_text());
}
