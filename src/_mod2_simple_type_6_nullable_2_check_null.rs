pub(crate) fn test() {
    let n1= Some(42);
    let exists1 = n1.is_some();
    //exists1 is true

    let n2: Option<i32> = None;
    let exists2 = n2.is_some();
    //exists2 is false

    let not_exists = n2.is_none();
    //not_exists is true

    println!("exists1 is {exists1}");
    println!("exists2 is {exists2}");
    println!("not_exists is {not_exists}");

}