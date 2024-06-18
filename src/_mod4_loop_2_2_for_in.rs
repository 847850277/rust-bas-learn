pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut sum = 0;
    for i in numbers {
        sum += i;
    }
    //sum is 77
    println!("sum is {sum}");

}