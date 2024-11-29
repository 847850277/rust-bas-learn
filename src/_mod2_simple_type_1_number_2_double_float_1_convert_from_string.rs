pub(crate) fn test() {
    //the first method
    let str_pi = "3.14";
    let pi_float: f32 = str_pi.parse().unwrap();

    //the second method
    let str_exp = "2.71828";
    let exp = str_exp.parse::<f64>().unwrap();

    //the third method
    let str_half = "0,5";
    let half = str_half.replace(",", ".").parse::<f32>().unwrap();

    println!("pi_float is {pi_float}");
    println!("exp is {exp}");
    println!("half is {half}");
}
