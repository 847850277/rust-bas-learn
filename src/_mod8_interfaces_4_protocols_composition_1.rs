trait PId {
    fn get_id(&self) -> i32;
}
trait Priced {
    fn get_price(&self) -> f64;
}
struct Goods {
    id: i32,
    price: f64,
}
impl PId for Goods {
    fn get_id(&self) -> i32 {
        return self.id;
    }
}
impl Priced for Goods {
    fn get_price(&self) -> f64 {
        return self.price;
    }
}
//Protocols composition
trait PGood: PId + Priced {}
impl PGood for Goods {}
struct Product {
    info: Box<dyn PGood>,
}
impl Product {
    fn new(info: Box<dyn PGood>) -> Product {
        return Product { info: info };
    }
    fn show_info(&self) {
        println!(
            "id = {}, price = {}",
            self.info.get_id(),
            self.info.get_price()
        );
    }
}

pub(crate) fn test() {
    let bread = Goods { id: 1, price: 35.5 };
    let product = Product::new(Box::new(bread));
    product.show_info();
}
