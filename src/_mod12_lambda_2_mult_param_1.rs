pub(crate) fn test() {
    let avg_func = |a, b| (a + b) / 2;
    let avg = avg_func(3, 5);
    //avg is 4
    println!("avg is {avg}");
}
