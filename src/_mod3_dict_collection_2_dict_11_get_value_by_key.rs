use std::collections::HashMap;

pub(crate) fn test() {
    let dic = HashMap::from([(1, "A"), (2, "B"), (3, "A")]);
    let value = "A";
    let keys: Vec<_> = dic
        .iter()
        .filter_map(|(key, &val)| if val == value { Some(key) } else { None })
        .collect();
    //keys is [1, 3]
    println!("keys is {:?}", keys);
}
