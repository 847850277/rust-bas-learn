pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11, 13, 17];
    let mut sum = 0;
    numbers.iter().for_each(|i| {
        sum += i;
    });
    //sum is 58
    println!("sum is {sum}");
    println!();
    numbers.iter().enumerate().for_each(|(i, v)| {
        println!("numbers[{i}] = {v}");
    });
}
