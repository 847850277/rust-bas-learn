pub(crate) fn test() {
    let d1: i64 = i64::MAX;
    //d1 is 9223372036854775807
    let d2 = d1 as i32;
    //d2 is -1

    let d3 = 10;
    let d4: i16 = d3 as i16;
    //d4 is 10

    let d5 = i32::MAX;
    //d5 is 2147483647
    let d6: i64 = d5.into();
    //d6 is 2147483647

    println!("d1 is {d1}");
    println!("d2 is {d2}");
    println!("d3 is {d3}");
    println!("d4 is {d4}");
    println!("d5 is {d5}");
    println!("d6 is {d6}");
}
