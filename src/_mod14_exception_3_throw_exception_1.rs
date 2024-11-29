use std::io::ErrorKind;
struct Car {}
struct Seller {
    cars: Vec<Car>,
}
impl Seller {
    fn sell(&self) -> Result<(), ErrorKind> {
        if self.cars.len() == 0 {
            return Err(ErrorKind::NotFound);
        }
        //some implementation...
        return Ok(());
    }
}
pub(crate) fn test() {
    let seller = Seller { cars: vec![] };
    if let Err(_err) = seller.sell() {
        println!("No cars for sale");
    }

    let result = seller.sell();
    println!("{:?}", result);
    match result {
        Ok(_) => println!("Car sold successfully"),
        Err(e) => println!("Error selling car: {:?}", e),
    }
}
