pub(crate) fn test() {
    let numbers = [2, 3, 5, 7];

    // iterates by reference
    let mut str1 = String::new();
    for i in numbers.iter() {
        if str1.chars().count() > 0 {
            str1.push_str(", ");
        }
        str1.push_str(&i.to_string());
    }
    //str1 is '2, 3, 5, 7'

    // iterates by value
    let mut str2 = String::new();
    for i in numbers {
        if str2.chars().count() > 0 {
            str2.push_str(", ");
        }
        str2.push_str(&i.to_string());
    }

    println!("str1 is '{str1}'");
    println!("str2 is '{str2}'");
}
