use std::collections::HashSet;

pub(crate) fn test() {
    let chars = HashSet::from(['A', 'B', 'C', 'D']);
    let contain_a = chars.contains(&'A');
    //contain_a is true
    let contain_e = chars.contains(&'E');
    //contain_e is false
    let chars2 = HashSet::from(['A', 'B']);
    let contain_all = chars2.is_subset(&chars);
    //contain_all is true
    println!("contain_a is {contain_a}");
    println!("contain_e is {contain_e}");
    println!("contain_all is {contain_all}");
}
