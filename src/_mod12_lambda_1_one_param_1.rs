pub(crate) fn test() {
    let pow_of_two = |power| 2_i32.pow(power);
    let pow8 = pow_of_two(8);
    //pow8 is 256
    println!("pow8 is {pow8}");

}