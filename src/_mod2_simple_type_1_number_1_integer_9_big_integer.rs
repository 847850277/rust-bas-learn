pub(crate) fn test() {
    let a = i64::MAX;
    println!("{}", a);
    let b: u128 = 255;
    let c: u128 = 1000;
    let a1: u128 = (a as u128) * c;
    println!("{}", a1);
    let a2: u128 = (a1 + c) / b;
    println!("{}", a2);
    println!("{}", u128::MAX);
}
