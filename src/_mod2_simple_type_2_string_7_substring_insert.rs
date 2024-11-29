pub(crate) fn test() {
    let mut data_string = String::from("string");

    data_string.insert_str(0, "Sub");
    //data_string is "Substring"
    println!("data_string is \"{data_string}\"");

    data_string.insert_str(9, "!");
    //data_string is "Substring!"
    println!("data_string is \"{data_string}\"");

    data_string.insert_str(9, " inserting");
    //data_string is "Substring inserting!"
    println!("data_string is \"{data_string}\"");
}
