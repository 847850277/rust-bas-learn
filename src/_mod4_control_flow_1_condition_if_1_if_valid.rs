use rand::Rng;
pub(crate) fn test() {
    let latitude = get_latitude();
    let location: &str;
    if latitude == 0 {
        location = "Equator";
    } else if latitude == 90 {
        location = "north Pole";
    } else if latitude == -90 {
        location = "south Pole";
    } else {
        location = "not at the Equator or Pole";
    }
    println!("latitude is {latitude}");
    println!("location is '{location}'");


}

fn get_latitude() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(-90..91);
}