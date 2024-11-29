pub(crate) fn test() {
    //to int
    let str_number = "42";
    //the first method
    let number1: i32 = str_number.parse().unwrap();
    //the second method
    let number2 = str_number.parse::<i32>().unwrap();

    //to Double and Float
    //the first method
    let str_pi = "3.14";
    let pi: f32 = str_pi.parse().unwrap();

    //the second method
    let str_exp = "2.71828";
    let exp = str_exp.parse::<f64>().unwrap();

    //the third method
    let str_half = "0,5";
    let half = str_half.replace(",", ".").parse::<f32>().unwrap();

    println!("number1 is {number1}");
    println!("number2 is {number2}");
    println!("pi is {pi}");
    println!("exp is {exp}");
    println!("half is {half}");
}
