use std::collections::HashSet;

pub fn test() {
    let int_set = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19]);

    println!("int_set is {:?}", int_set);
    println!("{}", int_set.contains(&17));
    dbg!(int_set);
}
