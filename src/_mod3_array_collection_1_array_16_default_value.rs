pub(crate) fn test() {
    let value = 5;
    const COUNT: usize = 3;
    let array: [i32; COUNT] = [value; COUNT];
    //array is [5, 5, 5]
    println!("array is {:?}", array);

}