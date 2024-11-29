use std::collections::HashMap;
pub(crate) fn test() {
    let dic = HashMap::from([(1, "one"), (2, "two")]);
    let keys = dic.keys();
    //keys is [2, 1]
    println!("keys is {:?}", keys);
}
