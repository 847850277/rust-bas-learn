pub fn test() {

    let d1 = 8 + 2;  //d1 is 10
    let d2 = 8 - 2;  //d2 is 6
    let d3 = 8 * 2;  //d3 is 16
    let d4 = 8 / 2;  //d4 is 4
    let d5 = 5 % 2;  //d5 is 1
    let d6 = -5 % 2; //d6 is -1
    let mut d7 = 1;
    d7 += 1;         //d7 is 2
    dbg!(d7);
    d7 -= 1;         //d7 is 1
    dbg!(d7);
    let (d8, d7) = (d7, d7 + 1);
    //d8 is 1, d7 is 2
    let (d9, d7) = (d7 + 1, d7 + 1);
    //d9 is 3, d7 is 3

    println!("d1 is {d1}");
    println!("d2 is {d2}");
    println!("d3 is {d3}");
    println!("d4 is {d4}");
    println!("d5 is {d5}");
    println!("d6 is {d6}");
    println!("d7 is {d7}");
    println!("d8 is {d8}");
    println!("d9 is {d9}");

    dbg!(d1);
    dbg!(d2);
    dbg!(d3);
    dbg!(d4);
    dbg!(d5);
    dbg!(d6);
    dbg!(d7);
    dbg!(d8);
    dbg!(d9);

}