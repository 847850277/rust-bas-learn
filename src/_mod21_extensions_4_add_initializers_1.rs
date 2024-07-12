//struct has no properties in Rust
struct Size {
    width: i32,
    height: i32,
}
impl Size {
    fn new(width: i32, height: i32) -> Size {
        return Size {
            width: width,
            height: height
        };
    }
}

pub(crate) fn test() {

    let size = Size::new(4, 5);
    //size is (4, 5)
    println!("size is ({}, {})", size.width, size.height);

}