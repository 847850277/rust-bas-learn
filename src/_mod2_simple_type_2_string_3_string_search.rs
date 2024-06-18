pub(crate) fn test() {
    let data_string = "Substring search";

    if data_string.contains("string") {
        println!("data_string contain \"string\"");
    }

    if data_string.starts_with("Sub") {
        println!("dataString starts with \"Sub\"");
    }

    if data_string.ends_with("search") {
        println!("dataString ends with \"search\"");
    }

}