pub(crate) fn test() {
    let list = [1, 2, 3];
    let iterator = list.iter();
    for x in iterator {
        println!("x is {:?}", x);
    }
}
