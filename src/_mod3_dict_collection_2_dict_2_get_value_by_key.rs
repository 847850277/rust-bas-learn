use std::collections::HashMap;
pub(crate) fn test() {
    let dic = HashMap::from([(3, "three"), (4, "four")]);
    let three = dic[&3];
    //three is "three"
    let four = dic.get(&4);
    //four is Some("four")
    let five = dic.get(&5);
    //five is None
    println!("three is '{:?}'", three);
    println!("four is {:?}", four);
    println!("five is {:?}", five);
}
