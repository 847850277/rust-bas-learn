pub(crate) fn test() {
    //range check by default
    let b1: u8 = 200;
    let b2: u8 = 100;
    let b3: u8 = b1 + b2; //<-Error

    println!("{}", b1);
    println!("{}", b2);
    println!("{}", b3);

}