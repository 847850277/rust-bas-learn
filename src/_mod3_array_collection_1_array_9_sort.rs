pub(crate) fn test() {
    let mut numbers = [11, 2, 5, 7, 3];
    numbers.sort();
    //numbers is [2, 3, 5, 7, 11]
    println!("numbers is {:?}", numbers);
    numbers.sort_by(|a, b| b.cmp(a));
    //numbers is [11, 7, 5, 3, 2]
    println!("numbers is {:?}", numbers);
}
