fn make_mult() -> impl Fn(i32, i32) -> i32 {
    return |a, b| -> i32 { a * b };
}

pub(crate) fn test() {
    let make_sum = || |a, b| a + b;
    let sum_func = make_sum();
    let sum = sum_func(5, 8);
    //sum is 13
    println!("sum is {sum}");
    let mult_func = make_mult();
    let mult = mult_func(5, 8);
    //mult is 40
    println!("mult is {mult}");

    // Fn: Closure captures by reference (&T)
    // FnMut: Closure captures by mutable reference (&mut T)
    // FnOnce: Closure captures by value (T)
}
