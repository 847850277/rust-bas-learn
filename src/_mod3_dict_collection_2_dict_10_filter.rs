use std::collections::HashMap;

pub(crate) fn test() {


    let mut dic = HashMap::from([
        (1, "one"),  (2, "two"), (3, "three")
    ]);
    let odd_dic: HashMap<_, _> = dic
        .iter().filter(|(&k, _v)| k % 2 == 1).collect::<HashMap<_, _>>();
    //odd_dic is {3: "three", 1: "one"}
    println!("odd_dic is {:?}", odd_dic);


}