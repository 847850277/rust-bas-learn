use std::collections::HashMap;
pub(crate) fn test() {
    let dic = HashMap::from([(1, "one"), (2, "two")]);
    let values = dic.values();
    //values is ["two", "one"]
    println!("values is {:?}", values);
}
