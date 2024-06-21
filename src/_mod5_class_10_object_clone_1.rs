//There are no classes in Rust
#[derive(Copy, Clone)]
struct Shape<'a> {
    line_count: i32,
    name: &'a str
}
impl<'a> Shape<'a> {
    fn to_string(&self) -> String {
        return format!("{}, lineCount is {}",
                       self.name, self.line_count);
    }
}
pub(crate) fn test() {


    let mut square = Shape {
        line_count: 4, name: "Square" };
    println!("{}", square.to_string());
    //Square, lineCount is 4
    let clone = square;
    square.line_count = 5;
    println!("{}", clone.to_string());
    //Square, lineCount is 4
    println!("{}", square.to_string());
    //Square, lineCount is 5


}