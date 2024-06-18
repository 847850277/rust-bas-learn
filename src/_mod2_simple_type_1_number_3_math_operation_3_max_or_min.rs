pub(crate) fn test() {
    let min_v = std::cmp::min(
        std::cmp::min(2, 1), 3);
    //min_v is 1

    let max_v = [2, 1, 3].iter()
        .max().unwrap();
    //max_v is 1

    println!("min_v is {min_v}");
    println!("max_v is {max_v}");

}