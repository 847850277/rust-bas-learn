pub(crate) fn test() {
    let numbers = vec!["one", "two", "three"];
    let number_list = numbers.join("; ");
    // number_list is "one; two; three"

    println!("number_list is \"{number_list}\"");

}