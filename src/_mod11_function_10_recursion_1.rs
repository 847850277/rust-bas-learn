pub(crate) fn test() {
    let f10 = fibonacci(10);
    //f10 is 55
    println!("f10 is {f10}");

}

fn fibonacci(x: i32) -> i32 {
    return if x <= 2 { 1 } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    };
}