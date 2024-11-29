struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn move_to(&mut self, right: i32, down: i32) {
        self.x += right;
        self.y += down;
    }
    fn to_text(&self) -> String {
        return format!("x = {}; y = {}", self.x, self.y);
    }
}

pub(crate) fn test() {
    let mut p = Point { x: 1, y: 2 };
    let str1 = p.to_text();
    //str1 is "x = 1; y = 2"
    p.move_to(5, -1);
    let str2 = p.to_text();
    //str2 is "x = 6; y = 1"
    println!("str1 is '{str1}'");
    println!("str2 is '{str2}'");
}
