use std::collections::HashMap;
pub(crate) fn test() {
    //first method
    let d1 = HashMap::from([(1, "one"), (2, "two")]);
    let d2 = HashMap::from([(3, "three")]);
    let d_all1: HashMap<_, _> = d1.into_iter().chain(d2).collect();
    //d_all1 is {2: "two", 1: "one", 3: "three"}
    //second method
    let d3 = HashMap::from([(1, "one"), (2, "two")]);
    let mut d_all2 = HashMap::from([(3, "three")]);
    d_all2.extend(d3);
    //d_all2 is {2: "two", 1: "one", 3: "three"}
    println!("d_all1 is {:?}", d_all1);
    println!("d_all2 is {:?}", d_all2);
}
