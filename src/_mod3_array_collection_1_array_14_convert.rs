pub(crate) fn test() {
    let numbers = [1, 2, 3, 4, 5];
    let numbers3 = numbers.map(|n| n * 3);
    //numbers3 is [ 3, 6, 9, 12, 15 ]
    println!("numbers3 is {:?}", numbers3);

}