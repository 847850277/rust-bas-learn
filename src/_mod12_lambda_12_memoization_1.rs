use std::collections::HashMap;

fn fibonacci(x: i32) -> i32 {
    return if x <= 2 { 1 } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    };
}

pub(crate) fn test() {
    let memoize = |fun: Box<dyn Fn(i32) -> i32>| {
        let mut memo = HashMap::<i32, i32>::new();
        move |x: &i32| {
            if memo.contains_key(x) {
                return memo[x];
            }
            let r = fun(*x);
            let v = *x;
            memo.insert(v, r);
            return r;
        }
    };
    let mut mem_fibonacci = memoize(Box::new(fibonacci));
    for i in 1..3 {
        let start = chrono::offset::Utc::now();
        let f37 = mem_fibonacci(&37);
        let now = chrono::offset::Utc::now();
        let milliseconds = (now - start).num_milliseconds();
        println!("{i}: f37 is {f37}");
        println!("{i}: milliseconds is {milliseconds}");
    }
    // prints:
    // 1: f37 is 24157817
    // 1: milliseconds is 268
    // 2: f37 is 24157817
    // 2: milliseconds is 0
    let start = chrono::offset::Utc::now();
    let f38 = mem_fibonacci(&38);
    let now = chrono::offset::Utc::now();
    let milliseconds = (now - start).num_milliseconds();
    println!("f38 is {f38}");
    println!("milliseconds is {milliseconds}");
    // f38 is 39088169
    // milliseconds is 376

}

