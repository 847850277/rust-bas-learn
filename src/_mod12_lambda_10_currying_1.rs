pub(crate) fn test() {
    let carry = |f: Box<dyn Fn(i32, i32) -> i32>|
        |a| move |b| f(a, b);
    let avg = |a, b| (a + b)/2;
    let n1 = avg(1, 3);
    //n1 is 2
    println!("n1 is {n1}");
    let avg1 = carry(Box::new(avg))(1);
    //avg1 is avg func with first param = 1
    let n2 = avg1(5);
    //n2 is 3 = (1 + 5)/2
    println!("n2 is {n2}");

}