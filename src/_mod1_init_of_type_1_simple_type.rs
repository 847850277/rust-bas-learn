pub fn test(){
    //"const" for constants and
    //"let" for variables

    //Int
    let number = 42;
    let other_number = 37;
    const MAX_INT64 :i64 = i64::MAX;
    const MB: i32 = 1_048_576;
    //Double
    const EXP: f64 = 2.71828;
    let billion = 1E+9;

    //Float
    const PI: f32 = 3.14;

    //String
    let greeting: &str = "Hello";
    //Multiline String
    let text = "this is some\n".to_owned() +
        "multiline text";

    //Multiline String
    let multi_line = "this is some\
    multiline text";

    //Bool
    const SUN_IS_STAR: bool = true;
    let earth_is_star = false;

    //Character "A"
    let char_a: char = 'A'; //'\u{0041}'

    println!("number is {}", number);
    println!("other_number is {}", other_number);
    println!("MAX_INT64 is {}", MAX_INT64);
    println!("Mb is {}", MB);
    println!("EXP is {}", EXP);
    println!("billion is {}", billion);
    println!("PI is {}", PI);
    println!("greeting is {}", greeting);
    println!("text is {}", text);
    println!("multi_line is {}", multi_line);
    println!("SUN_IS_STAR is {}", SUN_IS_STAR);
    println!("earth_is_star is {}", earth_is_star);
    println!("char_a is {}", char_a);
}