pub(crate) fn test() {
    const COUNT: usize = 15;
    let mut ar_int: [i32; COUNT] = [0; COUNT];
    ar_int[0] = 1;
    //arInt1 is [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    print!("ar_int is {:?}", ar_int);
}
