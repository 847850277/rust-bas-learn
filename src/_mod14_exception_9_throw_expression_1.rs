use rand::Rng;
pub(crate) fn test() {

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(-3..3);
    let classify = if n > 0
    { "positive" } else { panic!("negative!") };
    //classify is "negative"
    println!("classify is '{classify}'");

}