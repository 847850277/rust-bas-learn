//struct has no properties in Rust
struct Size {
    width: i32,
    height: i32,
}
impl Size {
    fn area(self) -> i32 {
        return self.width * self.height;
    }
}

pub(crate) fn test() {
    let size = Size {
        width: 4,
        height: 5,
    };
    let area = size.area();
    //area is 20
    println!("area is {area}");
}
