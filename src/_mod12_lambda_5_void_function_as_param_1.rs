pub(crate) fn test() {
    let lambda_1 = |number, process: &dyn Fn(i32)| {
        if number < 10 {
            process(number);
        } else {
            println!(">= 10")
        }
    };
    let lambda_2 = |number| print!("{}", number * 10);
    lambda_1(5, &lambda_2);
    //printed: 50
}
