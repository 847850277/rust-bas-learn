use std::collections::HashSet;
pub(crate) fn test() {

    let set = HashSet::from(
        ["A", "B", "C", "D"]);
    let mut str1 = String::new();
    for i in set.iter() {
        if str1.chars().count() > 0 {
            str1.push_str("; ");
        }
        str1.push_str(&i.to_string());
    }
    //str1 is "A; D; C; B"
    println!("str1 is {:?}", str1);

}