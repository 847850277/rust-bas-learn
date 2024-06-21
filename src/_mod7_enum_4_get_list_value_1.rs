use strum::IntoEnumIterator;
use strum_macros::EnumIter;
#[derive(Debug, EnumIter)]
enum Season {
    Summer, Fall, Winter, Spring
}
pub(crate) fn test() {

    let values: Vec<Season> =
        Season::iter().collect();
    println!("values is {:?}", values);
}