pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11];
    let (first2, last3) = numbers
        .split_at(2);
    //first2 is [ 2, 3 ]
    //last3 is [ 5, 7, 11 ]
    let first3: Vec<&i32> = numbers
        .iter().take(3).collect();
    //first3 is [2, 3, 5]
    let first4 = &numbers[..4];
    //first4 is [2, 3, 5, 7]
    let last2 = &numbers[3..];
    //last2 is [7, 11]
    println!("first2 is {:?}", first2);
    println!("last3 is {:?}", last3);
    println!("first3 is {:?}", first3);
    println!("first4 is {:?}", first4);
    println!("last2 is {:?}", last2);

}