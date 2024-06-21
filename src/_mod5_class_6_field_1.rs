//There are no classes in Rust
struct Game  {
    pub name: String,
    pub year: i16,
    //Private to other modules
    developer: String
}
pub(crate) fn test() {

    let game = Game {
        name: String::from("S.T.A.L.K.E.R."),
        year: 2007,
        developer: String::from("GSC")
    };
    println!("name is {}", game.name);
    println!("year is {}", game.year);
    println!("developer is {}", game.developer);

}