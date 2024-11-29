use rustils::parse::boolean::string_to_bool;
pub(crate) fn test() {
    let str1 = "true";
    let value1 = str1.to_lowercase() == "true";
    //value1 is true

    let str2 = String::from("false");
    let value2 = string_to_bool(str2);
    //value1 is false

    println!("value1 is {value1}");
    println!("value2 is {value2}");
}
