#[allow(dead_code)]
#[derive(PartialEq, PartialOrd)]
enum Size {
    XS, S, M, L, XL
}
pub(crate) fn  test() {

    let small = Size::S;
    let lage = Size::L;
    println!("is l > s: {}", lage > small);

}