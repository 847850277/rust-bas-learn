pub(crate) fn test() {
    //with str
    let str = "1-3-2";
    let mut ar_str: Vec<char> = str.chars().collect();
    ar_str[2] = '2';
    ar_str[4] = '3';
    let str1 = ar_str.iter().collect::<String>();
    //str1 is "1-2-3"
    println!("str1 is \"{str1}\"");

    //with String
    let mut string = String::from("1-3-2");
    string.replace_range(2..3, "2");
    string.replace_range(4..5, "3");
    //string is "1-2-3"
    println!("string is \"{string}\"");
}
