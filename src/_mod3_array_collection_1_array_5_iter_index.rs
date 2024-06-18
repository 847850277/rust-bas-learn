pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11, 13, 17];

    // iterates by reference
    let mut str1 = String::new();
    for (i, x) in numbers.iter().enumerate() {
        if i > 0 {
            str1.push_str("; ");
        }
        str1.push_str(&x.to_string());
    }
    //str1 is '2; 3; 5; 7; 11; 13; 17'


    let mut str2 = String::new();
    for i in 0..numbers.len() {
        if i > 0 {
            str2.push_str("; ");
        }
        str2.push_str(&numbers[i].to_string());
    }

    println!("str1 is '{str1}'");
    println!("str2 is '{str2}'");

}