pub(crate) fn test() {
    //decimal number system
    let decimal = 42;

    //octal number system
    let octal = 0o42;
    //octal is 34

    //hexadecimal number system
    let hex = 0x42;
    //hexadecimal is 66

    //binary number system
    let binary = 0b1010;
    //binary is 10

    //42 to decimal string
    let s_decimal = decimal.to_string();
    //sDecimal is "42"

    //42 to octal string
    let s_octal = format!("{:o}", decimal);
    //sOctal is "52"

    //42 to hexadecimal string
    let s_hex = format!("{:x}", decimal);
    //sHexadecimal is "2a"

    //42 to binary string
    let s_binary = format!("{:b}", decimal);
    //sBinary is "101010"

    println!("decimal is {decimal}");
    println!("octal is {octal}");
    println!("hex is {hex}");
    println!("binary is {binary}");
    println!("s_decimal is {s_decimal}");
    println!("s_octal is {s_octal}");
    println!("s_hex is {s_hex}");
    println!("s_binary is {s_binary}");

}