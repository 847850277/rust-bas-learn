pub(crate) fn test() {
    let numbers = [1, 2, 3, 4, 5];
    let all_less10 = numbers.iter().all(|i| *i < 10);
    //all_less10 is true
    let some_more3 = numbers.iter().any(|i| *i > 3);
    //some_more3 is true
    let all_odd = numbers.iter().all(|i| *i % 2 == 1);
    //all_odd is false
    println!("all_less10 is {all_less10}");
    println!("some_more3 is {some_more3}");
    println!("all_odd is {all_odd}");
}
