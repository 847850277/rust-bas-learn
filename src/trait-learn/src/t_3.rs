// trait定义共有行为

// trait定义共有行为,通过方法可以为类型提供共有行为,这样就可以在不同的类型上调用相同的方法,而不用担心类型是否支持这个方法

pub struct Book {
    pub name: String,
    pub author: String,
    pub price: f32,
    pub inventory: i32,
}

pub struct Cosmetics {
    pub name: String,
    pub brand: String,
    pub price: f32,
    pub inventory: i32,
}

pub trait Record{
    fn set_price(&mut self, price: f32);

    fn set_inventory(&mut self, inventory: i32);
}

impl Record for Book{
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn set_inventory(&mut self, inventory: i32) {
        self.inventory = inventory;
    }
}

impl Record for Cosmetics{
    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn set_inventory(&mut self, inventory: i32) {
        self.inventory = inventory;
    }
}

pub fn test(){
    let mut book = Book{
        name: "Rust".to_string(),
        price: 100.0,
        inventory: 10,
        author: "Tom".to_string(),
    };

    let mut cosmetics = Cosmetics{
        name: "Lipstick".to_string(),
        price: 50.0,
        inventory: 20,
        brand: "Chanel".to_string(),
    };

    // print book info
    println!("Book: name: {}, price: {}, inventory: {}, author: {}", book.name, book.price, book.inventory, book.author);
    book.set_price(200.0);
    book.set_inventory(5);
    println!("Book: name: {}, price: {}, inventory: {}, author: {}", book.name, book.price, book.inventory, book.author);

    //print cosmetics info
    println!("Cosmetics: name: {}, price: {}, inventory: {}, brand: {}", cosmetics.name, cosmetics.price, cosmetics.inventory, cosmetics.brand);
    cosmetics.set_price(100.0);
    cosmetics.set_inventory(10);
    println!("Cosmetics: name: {}, price: {}, inventory: {}, brand: {}", cosmetics.name, cosmetics.price, cosmetics.inventory, cosmetics.brand);

    println!("size of book: {}", std::mem::size_of_val(&book));

}