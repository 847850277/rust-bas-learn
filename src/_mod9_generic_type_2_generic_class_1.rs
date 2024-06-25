//There are no classes in Rust
struct Size<T> {
    width: T,
    height: T
}
impl<T: std::fmt::Debug> Size<T> {
    fn new(width: T, height: T) -> Size<T> {
        return Size {
            width: width, height: height
        };
    }
    fn as_text(&self) -> String {
        return format!("[{:?}, {:?}]", self.width, self.height);
    }
}

pub(crate) fn test() {

    let size_int = Size::new(5, 8);
    let text_int = size_int.as_text();
    //text_int is "[5; 8]"
    let size_float = Size::new(3.7, 1.58);
    let text_float = size_float.as_text();
    //text_float is "[3.7; 1.58]"
    println!("text_int is {text_int}");
    println!("text_float is {text_float}");

}