pub fn test() {
    let str_number = "42";
    let number1: i8 = str_number.parse().unwrap();
    //number1 is 42

    let number2 = str_number.parse::<i16>().unwrap();
    //number2 is 42

    println!("number1 is {number1}");
    println!("number2 is {number2}");

    dbg!(number1);

    dbg!(number2);
}
