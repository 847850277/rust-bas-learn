#[derive(Debug)]
#[allow(dead_code)]
struct ColorPoint {
    //Fields
    x: i32,
    y: i32,
    //In Rust there are no properties
    color: String
}

pub(crate) fn test() {

    let p = ColorPoint{
        x: 1, y: 2,
        color: "red".to_string()};
    println!("p is '{:?}'", p);

}