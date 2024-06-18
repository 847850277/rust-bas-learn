use std::collections::HashMap;
pub(crate) fn test() {

    let dic = HashMap::from([
        (1, "one"),  (2, "two")]);
    let mut str1 = String::new();
    for (key, value) in &dic {
        if str1.chars().count() > 0 {
            str1.push_str(", ");
        }
        str1.push_str(
            &format!("({key}, \"{value}\")"));
    }
    //str1 is '(1, "one"), (2, "two")'
    println!("str1 is '{str1}'");

}