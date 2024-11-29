pub(crate) fn test() {
    main();
}

//abstract factory
trait Factory {
    fn create_a(&self) -> Box<dyn ProductA>;
    fn create_b(&self) -> Box<dyn ProductB>;
}
//concrete factory 1
struct Factory1 {}
impl Factory for Factory1 {
    fn create_a(&self) -> Box<dyn ProductA> {
        return Box::from(ProductA1 {});
    }
    fn create_b(&self) -> Box<dyn ProductB> {
        return Box::from(ProductB1 {});
    }
}
//concrete factory 2
struct Factory2 {}
impl Factory for Factory2 {
    fn create_a(&self) -> Box<dyn ProductA> {
        return Box::from(ProductA2 {});
    }
    fn create_b(&self) -> Box<dyn ProductB> {
        return Box::from(ProductB2 {});
    }
}
//abstract product A
trait ProductA {
    fn test_a(&self);
}
//abstract product B
trait ProductB {
    fn test_b(&self);
}
//concrete product A1
struct ProductA1 {}
impl ProductA for ProductA1 {
    fn test_a(&self) {
        println!("test A1");
    }
}
//concrete product A2
struct ProductA2 {}
impl ProductA for ProductA2 {
    fn test_a(&self) {
        println!("test A2");
    }
}
//concrete product B1
struct ProductB1 {}
impl ProductB for ProductB1 {
    fn test_b(&self) {
        println!("test B1");
    }
}
//concrete product B2
struct ProductB2 {}
impl ProductB for ProductB2 {
    fn test_b(&self) {
        println!("test B2");
    }
}
//client code
fn test_factory(factory: Box<dyn Factory>) {
    let product_a = factory.create_a();
    let product_b = factory.create_b();
    product_a.test_a();
    product_b.test_b();
}
fn main() {
    test_factory(Box::new(Factory1 {}));
    //printed: test A1
    //         test B1
    test_factory(Box::new(Factory2 {}));
    //printed: test A2
    //         test B2
}
