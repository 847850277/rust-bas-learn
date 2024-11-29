pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut str1 = String::new();
    for (i, v) in numbers.iter().enumerate() {
        if i % 2 == 1 {
            continue;
        }
        if !str1.is_empty() {
            str1.push_str("-");
        }
        str1.push_str(&v.to_string());
    }
    //str1 is "2-3-5-7"
    println!("str1 is '{str1}'");
}
