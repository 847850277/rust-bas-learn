pub(crate) fn test() {
    let name = "Alex";
    let name_exists = name
        .chars().count() > 1;
    //name exists is true

    let number = 7;
    let is_ten = number == 10;
    //isTen is false

    println!("name_exists is {name_exists}");
    println!("is_ten is {is_ten}");

}