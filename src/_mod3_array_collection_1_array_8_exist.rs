pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11, 13, 17];
    let contain2 = numbers.contains(&2);
    //contain2 is true
    let contain4 = numbers.contains(&4);
    //contain4 is false
    let index5 = numbers.iter().position(|&x| x == 5).unwrap();
    //index5 is 2
    println!("contain2 is {contain2}");
    println!("contain4 is {contain4}");
    println!("index5 is {index5}");
}
