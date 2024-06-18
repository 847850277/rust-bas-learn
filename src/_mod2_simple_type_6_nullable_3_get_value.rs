pub(crate) fn test() {
    let n1 = Some(42);
    let value1 = n1.unwrap();
    //value1 is 42

    let n2: Option<f32> = None;
    let value2 = n2.unwrap_or(3.14);
    //value2 is 3.14

    println!("value1 is {value1}");
    println!("value2 is {value2}");

}