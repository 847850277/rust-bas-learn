trait Named {
    fn print_type(&self) {
        println!("{}", std::any::type_name::<Self>())
    }
}
struct Flower;
struct City;
struct Star;
impl Named for Flower{}
impl Named for City{}
impl Named for Star{}

pub(crate) fn test() {

    let rows: [Box<dyn Named>; 3] = [
        Box::new(Flower{}),
        Box::new(City{}),
        Box::new(Star{})
    ];
    for named in rows {
        named.print_type();
    }
}