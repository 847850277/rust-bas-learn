use std::collections::HashMap;
pub(crate) fn test() {

    let mut map = HashMap::from([
        (1, "one"),  (2, "two")]);
    map.insert(3, "tree");
    //map is {1: "one", 2: "two", 3: "tree"}
    println!("map is {:?}", map);
    map.insert(3, "three");
    //map is {1: "one", 2: "two", 3: "three"}
    println!("map is {:?}", map);
    map.remove(&3);
    //map is {2: "two", 1: "one"}
    println!("map is {:?}", map);
    map.clear();
    //map is {}
    println!("map is {:?}", map);

}