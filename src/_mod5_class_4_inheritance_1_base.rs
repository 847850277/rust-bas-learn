struct Shape {
    line_count: i16,
}
struct Square {
    parent: Shape,
    side_length: i16,
}
impl Square {
    fn new(side_length: i16) -> Square {
        return Square {
            parent: Shape { line_count: 4 },
            side_length: side_length,
        };
    }
    fn line_count(&self) -> i16 {
        return self.parent.line_count;
    }
}

pub(crate) fn test() {
    let square = Square::new(5);
    println!("line_count is {}", square.line_count());
    println!("side_length is {}", square.side_length);
}
