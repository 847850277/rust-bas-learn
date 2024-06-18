use std::collections::HashMap;
pub(crate) fn test() {

    let map = HashMap::from([
        (1, "one"),  (2, "two")]);
    let upper_map: HashMap<_, _> = map.iter()
        .map(|x| (x.0, x.1.to_uppercase()))
        .collect();
    //upper_map is {1: "ONE", 2: "TWO"}
    println!("upper_map is {:?}", upper_map);

}