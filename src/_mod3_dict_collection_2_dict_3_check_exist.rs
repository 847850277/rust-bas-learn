use std::collections::HashMap;
pub(crate) fn test() {
    let dic = HashMap::from([(1, Some("one")), (2, None)]);
    let exists1 = dic.get(&1) != None;
    //exists1 is true
    let exists2 = dic.contains_key(&2);
    //exists2 is true
    let exists3 = dic.contains_key(&3);
    //exists3 is false
    println!("exists1 is '{exists1}'");
    println!("exists2 is '{exists2}'");
    println!("exists3 is '{exists3}'");
}
