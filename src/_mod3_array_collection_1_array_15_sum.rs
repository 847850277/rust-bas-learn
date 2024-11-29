pub(crate) fn test() {
    let numbers = [2, 3, 5, 7, 11];
    let n_sum: u8 = numbers.iter().sum();
    // nSum is 28
    let strings = ["A", "B", "C"];
    let s_sum = strings.join(",");
    // sSum is 'A,B,C'
    println!("n_sum is {n_sum}");
    println!("s_sum is '{s_sum}'");
}
