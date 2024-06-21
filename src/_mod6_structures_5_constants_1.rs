// Constants begin with an uppercase letter
struct BoldLine {}
impl BoldLine {
    const WIDTH: i8 = 10;
}

pub(crate) fn test() {

    let line_width = BoldLine::WIDTH;
    //line_width is 10
    //BoldLine::WIDTH = 5; //<-Error
    println!("line_width is {line_width}");

}