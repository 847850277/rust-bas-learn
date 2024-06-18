pub(crate) fn test() {
    let a: f64 = 1.0;
    let b: f64 = (0.3 * 3.0) + 0.1;

    //Wrong way to compare
    let is_equal1 = a == b;
    //is_equal1 is false
    dbg!(is_equal1);

    //Сorrect way to compare
    let delta = 1E-9;
    let is_equal2 = (a - b).abs() < delta;
    //is_equal2 is true

    println!("a is {a}");
    println!("b is {b}");
    println!("delta is {delta}");
    println!("is_equal1 is {is_equal1}");
    println!("is_equal2 is {is_equal2}");

}