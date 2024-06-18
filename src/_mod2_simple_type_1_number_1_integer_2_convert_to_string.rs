pub fn test() {
    let number = 42;
    let s1 = number.to_string();
    //s1 is "42"

    let s2 = format!("{number}");
    //s2 is "42"

    let s3 = format!("{:03}", number);
    //s3 is "042"

    println!("s1 is '{s1}'");
    println!("s2 is '{s2}'");
    println!("s3 is '{s3}'");

    dbg!(s1);
    dbg!(s2);
    dbg!(s3);

}