trait Shape{
    fn line_count(& self) -> i32{
        return 0;
    }
}

struct Square{
    
}

impl Shape for Square{
    fn line_count(&self) -> i32 {
        return 4;
    }

}


pub(crate) fn test() {
    let square = Square{};
    let line_count = square.line_count();
    println!("{}", line_count);
}