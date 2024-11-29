use std::collections::HashMap;
pub(crate) fn test() {
    let dic = HashMap::from([(1, "one"), (2, "two")]);
    let count = dic.len();
    //count is 2
    println!("count is {count}");
}
