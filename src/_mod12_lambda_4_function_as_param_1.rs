pub(crate) fn test() {
    let mut numbers = [2, 3, 1];
    let lambda = |a: &i32, b: &i32| b.cmp(a);
    numbers.sort_by(lambda);
    //numbers is [3, 2, 1]
    println!("numbers is {:?}", numbers);
}
