use std::collections::HashMap;
pub(crate) fn test() {
    // Empty dictionary
    let d1 = HashMap::<u8, &str>::new();
    // init with some data
    let d2 = HashMap::from([(1, "one"), (2, "two")]);
    println!("d1 is {:?}", d1);
    println!("d2 is {:?}", d2);
}
