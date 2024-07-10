use std::collections::HashMap;
//Flyweight
trait PSpan {
    fn print_span(&self, style: &str);
}
//ConcreteFlyweight
#[derive(PartialEq, Eq)]
struct Char {
    c: char
}
impl PSpan for Char {
    fn print_span(&self, style: &str) {
        println!("<span style=\"{}\">{}</span>",
                 style, self.c);
    }
}
//FlyweightFactory
struct CharFactory {
    chars: HashMap::<char, Char>
}
impl CharFactory {
    //GetFlyweight(key)
    fn get_char(&mut self, c: char) -> &Char {
        if !self.chars.contains_key(&c) {
            let character = Char{ c: c };
            self.chars.insert(c, character);
        }
        return &self.chars[&c];
    }
}

pub(crate) fn test() {

    //Client
    let mut factory = CharFactory {
        chars: HashMap::<char, Char>::new() };
    let char_a = factory.get_char('A');
    char_a.print_span("font-size: 12");
    let char_b = factory.get_char('B');
    char_b.print_span("font-size: 12");
    let char_a1 = factory.get_char('A');
    char_a1.print_span("font-size: 14");

}