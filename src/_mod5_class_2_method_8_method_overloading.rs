struct foo {}

impl foo {
    fn show_params<T: ToString>(v: T) {
        println!("v is {}", v.to_string());
    }
}
pub(crate) fn test() {
    foo::show_params("A");
    foo::show_params(66);
    foo::show_params(true);
}
