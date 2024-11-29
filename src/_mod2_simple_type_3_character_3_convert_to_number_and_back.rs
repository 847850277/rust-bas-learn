pub(crate) fn test() {
    let char_a = 'A';
    let mut int_value = u32::from(char_a);
    //intValue is 65
    println!("char_a is '{char_a}'");
    println!("int_value is {int_value}");

    int_value += 1;
    let char_b = char::from_u32(int_value).unwrap();
    //charB is 'B'
    println!("char_b is '{char_b}'");
    println!("int_value is {int_value}");
}
