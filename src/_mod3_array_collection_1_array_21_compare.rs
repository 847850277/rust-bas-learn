pub(crate) fn test() {
    use std::collections::HashSet;
    let ar1 = [1, 2, 4, 3];
    let ar2 = [1, 2, 3, 4, 5];
    let set1 = HashSet::from(ar1);
    let set2 = HashSet::from(ar2);
    let diff = set2.difference(&set1);
    //diff is [5]
    println!("diff is {:?}", diff);
}
