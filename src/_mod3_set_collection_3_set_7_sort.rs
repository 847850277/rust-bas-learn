use itertools::Itertools;

pub(crate) fn test() {
    use std::collections::HashSet;
    //first method
    let set1 = HashSet::from([1, 2, 3, 4]);
    let even_set: HashSet<_> = set1.iter()
        .filter(|&&i| i % 2 == 0).collect();
    //even_set is {4, 2}
    let mut set2 = HashSet::from([1, 2, 3]);
    let mut odd_set: Vec<_> = set2.clone().into_iter().collect();
    odd_set.sort_by(|a, b| b.cmp(a));
    //odd_set is {1, 3}
    //set2 is {2}
    println!("even_set is {:?}", even_set);
    println!("odd_set is {:?}", odd_set);
    println!("even_set2 is {:?}", set2);

}