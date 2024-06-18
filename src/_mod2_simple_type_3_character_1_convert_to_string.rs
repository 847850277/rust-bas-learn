pub(crate) fn test() {
    let char_a = 'A';
    let string_a = char_a.to_string();
    //string_a is "A"

    let mut s: String = "character ".to_string();
    s.push(char_a);
    //s is "character A"

    println!("string_a is '{string_a}'");
    println!("s is '{s}'");

}