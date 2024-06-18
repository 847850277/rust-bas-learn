use std::collections::HashSet;
pub(crate) fn test() {

    let first = HashSet::from([1, 2]);
    let second = HashSet::from([1, 2]);
    let third = HashSet::from([1, 2, 3]);
    let is_equal1 = first == second;
    //is_equal1 is true
    let is_equal2 = first == third;
    //is_equal2 is false
    let is_intersects = !first.is_disjoint(&third);
    //is_intersects is true
    let is_subset = third.is_subset(&first);
    //is_subset is false
    let is_superset = third.is_superset(&first);
    //is_superset is true
    println!("is_equal1 is {is_equal1}");
    println!("is_equal2 is {is_equal2}");
    println!("is_intersects is {is_intersects}");
    println!("is_subset is {is_subset}");
    println!("is_superset is {is_superset}");

}