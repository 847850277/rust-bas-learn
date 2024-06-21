//There are no classes in Rust
struct SomeClass;
impl SomeClass {
    fn show_name(&self) {
        println!("struct: SomeClass");
    }
}

pub(crate) fn test() {

    let some_class = SomeClass;
    some_class.show_name();

}