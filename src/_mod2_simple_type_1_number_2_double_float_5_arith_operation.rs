pub(crate) fn test() {
    let d1 = 8.5 + 2.4;         // d1 is 10.9
    let d2 = 8.5 - 2.4;         // d2 is 6.1
    let d3 = 8.5 * 2.0;         // d3 is 17
    let d4 = 8.5 / 2.0;         // d4 is 4.25
    // mod
    let d5 = 7.5 % 2.0;         // d5 is 1.5
    let d6 = -7.5 % 2.0;        // d6 is -1.5
    // div
    let d7 = (7.5 as i16) / 2;  // d7 is 3
    let d8 = -d7;               // d8 is -3
    let mut d9 = 3.5;
    d9 += 1.0;                  // d9 is 4.5
    d9 -= 1.0;                  // d9 is 3.5
    let d10 = (-5.5_f32).abs(); // d10 is 5.5

    println!("d1 is {d1}");
    println!("d2 is {d2}");
    println!("d3 is {d3}");
    println!("d4 is {d4}");
    println!("d5 is {d5}");
    println!("d6 is {d6}");
    println!("d7 is {d7}");
    println!("d8 is {d8}");
    println!("d9 is {d9}");
    println!("d10 is {d10}");

}