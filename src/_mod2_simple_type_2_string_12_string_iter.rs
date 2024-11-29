pub(crate) fn test() {
    let str = "level";

    for c in str.chars() {
        println!("{c}");
    }

    //Iterating with index
    for (i, c) in str.chars().enumerate() {
        println!("str[{i}] = {c};");
    }
}
