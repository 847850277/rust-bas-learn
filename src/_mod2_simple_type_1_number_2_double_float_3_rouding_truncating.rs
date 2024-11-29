pub(crate) fn test() {
    let pi = 3.1415_f64;
    let pi_round = (pi * 1000.0).round() / 1000.0;
    //pi_round is 3.142

    let pi_trunc = (pi * 1000.0).floor() / 1000.0;
    //pi_trunc is 3.141

    let pi_ceil = (pi * 100.0).ceil() / 100.0;
    //pi_ceil is 3.15

    println!("pi_round is {pi_round}");
    println!("pi_trunc is {pi_trunc}");
    println!("pi_ceil is {pi_ceil}");
}
