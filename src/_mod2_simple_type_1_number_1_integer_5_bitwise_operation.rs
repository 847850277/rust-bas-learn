pub(crate) fn test() {
    let a = 5; //0101
    let b = 6; //0110

    //And
    let c1 = a & b;
    //c1 is 4 (0100)

    //Or
    let c2 = a | b;
    //c2 is 7 (0111)

    //Xor
    let c3 = a ^ b;
    //c3 is 3 (0011)

    //shift right
    let c4 = a >> 1;
    //c4 is 2 (0010)

    //shift left
    let c5 = b << 1;
    //c5 is 12 (1100)

    //bits invertion
    let c6 = !b;
    //c6 is -7 (-111)

    println!("c1 is {c1}");
    println!("c2 is {c2}");
    println!("c3 is {c3}");
    println!("c4 is {c4}");
    println!("c5 is {c5}");
    println!("c6 is {c6}");
}
