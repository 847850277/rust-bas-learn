pub(crate) fn test() {
    let numbers = [2, 3, 5, 7];
    // iterates by reference
    let mut str1 = String::new();
    for i in numbers.iter().rev() {
        if str1.chars().count() > 0 {
            str1.push_str(", ");
        }
        str1.push_str(&i.to_string());
    }
    //str1 is '7, 5, 3, 2'
    println!("str1 is '{str1}'");
}
