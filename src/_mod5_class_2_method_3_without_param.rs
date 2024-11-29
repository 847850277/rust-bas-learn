struct Greeting {}
impl Greeting {
    fn say_goodby(self) {
        println!("Goodby!");
    }
}
pub(crate) fn test() {
    let greeting = Greeting {};
    greeting.say_goodby();
}
