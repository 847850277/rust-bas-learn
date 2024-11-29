use std::collections::{HashMap, HashSet};
pub(crate) fn test() {
    //List of integer
    let int_list: Vec<i8> = vec![5];
    //Dictionary
    let dic: HashMap<u8, &str> = HashMap::from([(1, "one"), (2, "two")]);
    //Set
    let set: HashSet<i16> = HashSet::from([2, 3, 5]);
    println!("int_list is {:?}", int_list);
    println!("dic is {:?}", dic);
    println!("set is {:?}", set);
}
