use std::collections::HashSet;
pub(crate) fn test() {
    let int_set1 = HashSet::from([1, 2, 3]);
    let int_set2: HashSet<i16> = (1..4).collect();
    let str_set = HashSet::from(["Einar", "Olaf", "Harald"]);
    println!("int_set1 is {:?}", int_set1);
    println!("int_set2 is {:?}", int_set2);
    println!("str_set is {:?}", str_set);
}
