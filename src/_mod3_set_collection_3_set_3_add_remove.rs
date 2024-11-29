use std::collections::HashSet;
pub(crate) fn test() {
    let mut set = HashSet::from(["A", "B", "C"]);
    set.insert("D");
    //set is {"B", "C", "D", "A"}
    println!("set is {:?}", set);
    set.remove("A");
    //set is {"B", "C", "D", "A"}
    println!("set is {:?}", set);
    set.clear();
    //set is {}
    println!("set is {:?}", set);
}
