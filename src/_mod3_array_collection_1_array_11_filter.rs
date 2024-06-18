pub(crate) fn test() {
    let numbers = [1, 2, 3, 4, 5];
    let array_iter = numbers.iter();
    // 奇数
    let odds: Vec<&i32> = array_iter
        .filter(|&&i| i % 2 == 1).collect();
    print!("odds is {:?}", odds);

}