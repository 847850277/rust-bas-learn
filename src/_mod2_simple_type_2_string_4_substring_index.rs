pub(crate) fn test() {
    let data_string = "Substring index";
    // 结果为option
    let index = data_string.find("string");
    // index is 3

    match index {
        // The division was valid
        Some(x) => println!("index is {x}"),
        // The division was invalid
        None => println!("Substring not found"),
    }
}
