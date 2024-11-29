fn get_sum(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}

pub(crate) fn test() {
    let sum = get_sum(5, 3);
    //sum is 8
    println!("sum is {sum}");
}
