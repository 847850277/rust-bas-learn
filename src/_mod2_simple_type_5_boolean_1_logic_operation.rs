pub(crate) fn test() {
    let value1 = true;
    let value2 = false;

    let value_not1 = !value1;
    //valueNot1 is false

    let value_not2 = !value2;
    //valueNot2 is true

    let value_and = value1 && value2;
    //valueAnd is false

    let value_or = value1 || value2;
    //valueOr is true

    let value_xor = value1 ^ value2;
    //valueXor is true

    println!("value_not1 is {value_not1}");
    println!("value_not2 is {value_not2}");
    println!("value_and is {value_and}");
    println!("value_or is {value_or}");
    println!("value_xor is {value_xor}");

}