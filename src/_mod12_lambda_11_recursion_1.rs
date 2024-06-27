struct Fib<'s> { f: &'s dyn Fn(&Fib, i32) -> i32 }

pub(crate) fn test() {
    let fib = Fib {
        f: &|fib, x| if x <= 2 { 1 } else {
            (fib.f)(fib, x - 1) + (fib.f)(fib, x - 2)
        }
    };
    let f9 = (fib.f)(&fib, 9);
    // f9 is 34
    println!("f9 is {f9}");
    // currying
    let carry = |f: Box<dyn Fn(&Fib, i32) -> i32>|
        |a| move |b| f(a, b);
    let fibonacci = carry(Box::new(fib.f))(&fib);
    let f10 = fibonacci(10);
    // f10 is 55
    println!("{}", f10);
}
