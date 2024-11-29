pub(crate) fn test() {
    let s = "ABC";
    let char_a = s.chars().nth(0).unwrap();
    //char_a is 'A'
    let char_b = s.chars().nth(1).unwrap();
    //char_b is 'B'
    let char_c = s.chars().nth(2).unwrap();
    //char_c is 'C'

    let mut char_list: String = "".to_string();
    for c in s.chars() {
        char_list.push(c);
        char_list.push(';');
    }
    //char_list is "A;B;C;"

    println!("char_a is '{char_a}'");
    println!("char_b is '{char_b}'");
    println!("char_c is '{char_c}'");
    println!("char_list is '{char_list}'");
}
