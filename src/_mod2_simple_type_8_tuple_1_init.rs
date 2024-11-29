pub(crate) fn test() {
    let one = (1, "one");
    let number = one.0;
    // number is 1
    let str_v = one.1;
    // str_v is "one"

    println!("number is {number}");
    println!("str_v is '{str_v}'");
}
