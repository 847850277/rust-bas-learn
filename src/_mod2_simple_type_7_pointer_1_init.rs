pub(crate) fn test() {
    let number = Some(42);
    let none_int: Option<i32> = None;
    let pi: Option<f32> = Some(3.14);
    let char_a = Some('A');
    let has_value = Some(true);
    let no_value: Option<bool> = None;

    println!("number is {:?}", number);
    println!("none_int is {:?}", none_int);
    println!("pi is {:?}", pi);
    println!("char_a is {:?}", char_a);
    println!("has_value is {:?}", has_value);
    println!("no_value is {:?}", no_value);

}