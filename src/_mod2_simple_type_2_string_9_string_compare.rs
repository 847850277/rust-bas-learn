pub(crate) fn test() {
    let first = "A";
    let second = "B";
    let third = String::from("A");

    let are_equal1 = first == second;
    // are_equal1 is false

    let are_not_equal = first != second;
    // are_not_equal is true

    let are_equal2 = first == third;
    // are_equal2 is true

    let more_than = first > second;
    // more_than is false

    let compare1 = first.cmp(second);
    // compare1 is Less

    let compare2 = first.cmp(&third);
    // compare2 is Equal

    println!("are_equal1 is {are_equal1}");
    println!("are_not_equal is {are_not_equal}");
    println!("are_equal2 is {are_equal2}");
    println!("more_than is {more_than}");
    println!("compare1 is {:?}", compare1);
    println!("compare2 is {:?}", compare2);

}