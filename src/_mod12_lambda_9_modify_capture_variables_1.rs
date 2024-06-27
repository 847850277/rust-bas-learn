pub(crate) fn test() {
    let mut x = 5;
    let mut add_y_to_x = |y| x += y;
    add_y_to_x(3);
    //x is 8
    println!("x is {x}");

}