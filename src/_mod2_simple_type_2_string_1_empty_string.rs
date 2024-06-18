pub(crate) fn test() {
    let some_empty_string = "";
    let another_empty_string = String::new();

    if some_empty_string.is_empty() {
        println!("string is empty")
    }

    if another_empty_string.is_empty() {
        println!("another string is empty")
    }

}