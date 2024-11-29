pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11];
    let min = numbers.iter().min();
    //min is Some(2)
    let max = numbers.iter().max();
    //max is Some(11)
    println!("min is {:?}", min);
    println!("max is {:?}", max);
}
