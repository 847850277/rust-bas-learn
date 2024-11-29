use std::collections::HashSet;
pub(crate) fn test() {
    let first = HashSet::from([1, 2, 3]);
    let second = HashSet::from([3, 4, 5]);
    //union
    let s_union = first.union(&second);
    //s_union is [ 1, 2, 3, 4, 5 ]
    //difference
    let s_diff = first.difference(&second);
    //s_diff is [ 1, 2 ]
    //intersection
    let s_intersec = first.intersection(&second);
    //s_intersec is [ 3 ]
    //symmetric difference
    let s_sym_diff = first.symmetric_difference(&second);
    //s_sym_diff is [ 1, 2, 4, 5 ]
    println!("s_union is {:?}", s_union);
    println!("s_diff is {:?}", s_diff);
    println!("s_intersec is {:?}", s_intersec);
    println!("s_sym_diff is {:?}", s_sym_diff);
}
