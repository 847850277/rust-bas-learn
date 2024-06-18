pub(crate) fn test() {
    use std::collections::HashSet;
    let set = HashSet::from([1, 2, 3]);
    let set3: HashSet<_> = set.iter()
        .map(|&i| i*3).collect();
    //set3 is {3, 9, 6}
    //println!("set3 is {:?}", set3);
    dbg!(set3);
}