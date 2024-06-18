pub(crate) fn test() {
    let mut i = 5;
    let mut f5 = 1;
    while i > 1 {
        f5 *= i;
        i -= 1;
    }
    //f5 is 120
    println!("f5 is {f5}");

}