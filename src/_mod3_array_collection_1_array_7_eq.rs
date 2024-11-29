pub(crate) fn test() {
    let n1 = [1, 2, 3];
    let n2 = [1, 2, 3];
    let n3 = [3, 2, 1];
    let equal1 = n1 == n2;
    //equal1 is true
    let equal2 = n1 == n3;
    //equal2 is false
    println!("equal1 is {equal1}");
    println!("equal2 is {equal2}");
}
