trait Named {
    fn get_name(&self) -> String;
}
struct Flower {
    name: String,
}
impl Named for Flower {
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}
struct Weather;

fn test_1<T: Named>(s: &T) {
    println!("test passed");
}

pub(crate) fn test() {
    let rose = Flower {
        name: "Rose".to_string(),
    };
    let weather = Weather {};
    test_1(&rose);
    //test_1(&weather); //<-Error
    //I don't think there's a way to check whether a struct implements a trait at runtime. But you can check at compile time.
}
