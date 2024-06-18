use rand::Rng;

pub fn test() {


    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..3);
    //random is 0, 1 or 2
    println!("random is {random}");
    dbg!(rng);
    dbg!(random);
}