pub(crate) fn test() {
    let pi = std::f32::consts::PI;
    let sin90 = (pi / 2.0).sin();
    //sin90 is 1

    let cos180 = pi.cos();
    //cos180 is -1

    let tan45 = (pi / 4.0).tan();
    //tan45 is 1

    println!("sin90 is {sin90}");
    println!("cos180 is {cos180}");
    println!("tan45 is {tan45}");

}