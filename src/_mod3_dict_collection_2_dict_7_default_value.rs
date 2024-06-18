use std::collections::HashMap;
pub(crate) fn test() {

    let dic = HashMap::from([
        (1, "A"), (2, "B")
    ]);
    let value1 = dic.get(&3);
    //value1 is None
    let value2 = dic.get(&3)
        .cloned().unwrap_or("-");
    //value2 is "-"
    println!("value1 is {:?}", value1);
    println!("value2 is {:?}", value2);

}