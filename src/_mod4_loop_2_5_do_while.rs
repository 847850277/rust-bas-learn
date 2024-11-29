pub(crate) fn test() {
    let mut i = 7i32;
    let mut f7 = 1;
    while {
        f7 *= i;
        i -= 1;
        i > 1
    } {}
    //f7 is 5040
    println!("f7 is {f7}");
}
