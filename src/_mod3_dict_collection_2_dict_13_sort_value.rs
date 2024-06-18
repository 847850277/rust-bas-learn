use std::collections::HashMap;
pub(crate) fn test() {

    let dic = HashMap::from([
        (3, "B"), (1, "C"), (2, "A")
    ]);
    let mut sorted: Vec<_> = dic
        .iter().collect();
    sorted.sort_by_key(|a| a.1);
    //sorted is [(2, "A"), (3, "B"), (1, "C")]
    println!("sorted is {:?}", sorted);

}