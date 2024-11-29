//There are no classes in Rust
struct Settings;
impl Settings {
    const HOST: &'static str = "10.0.0.1";
    const PORT: i32 = 3306;
    fn get_connection() -> String {
        return format!("{}:{}", Settings::HOST, Settings::PORT);
    }
}

pub(crate) fn test() {
    let connection = Settings::get_connection();
    //connection is "10.0.0.1:3306"
    println!("connection is '{connection}'");
}
