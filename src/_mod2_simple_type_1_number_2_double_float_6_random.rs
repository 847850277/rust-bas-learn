use rand::Rng;

pub(crate) fn test() {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0.0..1.0);

    println!("random is {random}");
}
