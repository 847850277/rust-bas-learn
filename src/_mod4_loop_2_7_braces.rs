pub(crate) fn test() {
    let mut i: i32 = 0;
    loop {
        i += 1;
        println!("{i}^3 = {}", i.pow(3));
        if i == 5 {
            break;
        }
    }
    // 1^3 = 1
    // 2^3 = 8
    // 3^3 = 27
    // 4^3 = 64
    // 5^3 = 125
}
