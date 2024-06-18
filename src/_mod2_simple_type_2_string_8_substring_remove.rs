pub(crate) fn test() {
    let mut data_string = String::from("Substring removing!");

    //there is no "remove" method
    data_string.replace_range(9..18, "");
    //data_string is "Substring!"
    println!("data_string is \"{data_string}\"");

    data_string.replace_range(..3, "");
    //data_string is "string!"
    println!("data_string is \"{data_string}\"");

    // with str
    let data_str = "Substring removing!";
    let data_str1 = format!("{}{}",
                            &data_str[..9], &data_str[18..]);
    println!("data_str1 is \"{}\"", data_str1);

}