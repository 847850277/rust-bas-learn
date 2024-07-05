#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
struct Book {
    id: i32,
    format: String,
}
impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub(crate) fn test() {

    let p1 = Point{ x: 1, y: 1 };
    let p2 = Point{ x: 2, y: 2 };
    let p3 = Point{ x: 1, y: 1 };
    let b1 = Book { id: 1,
        format: "Paper".to_string() };
    let b2 = Book { id: 1,
        format: "Ebook".to_string() };
    let equal1 = p1 == p2;
//equal1 is false
    let equal2 = p1 == p3;
//equal2 is true
    let equal3 = p1 != p3;
//equal3 is false
    let equal4 = b1 == b2;
//equal3 is true
    println!("equal1 is {equal1}");
    println!("equal2 is {equal2}");
    println!("equal3 is {equal3}");
    println!("equal4 is {equal4}");

}