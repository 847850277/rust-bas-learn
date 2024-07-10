trait Text {
    fn get_text(&self) -> String;
    fn add_line(&mut self, value: &str);
}
//RefinedAbstraction
struct TextMaker {
    text_imp: Box<dyn ImplText>
}
impl Text for TextMaker {
    fn get_text(&self) -> String {
        return self.text_imp.get_string();
    }
    fn add_line(&mut self, value: &str) {
        self.text_imp.append_line(value);
    }
}
//Implementator
trait ImplText {
    fn get_string(&self) -> String;
    fn append_line(&mut self, value: &str);
}
//ConcreteImplementator
struct TextBuilder {
    rows: String
}
impl ImplText for TextBuilder {
    fn get_string(&self) -> String {
        return self.rows.clone();
    }
    fn append_line(&mut self, value: &str) {
        self.rows.push_str(value);
        self.rows.push_str("\n");
    }
}
//ConcreteImplementator
struct HtmlBuilder {
    rows: String
}
impl ImplText for HtmlBuilder {
    fn get_string(&self) -> String {
        return self.rows.clone();
    }
    fn append_line(&mut self, value: &str) {
        self.rows.push_str("<span>");
        self.rows.push_str(value);
        self.rows.push_str("</span><br/>\n");
    }
}
//Client
fn fill_text_builder(text_imp: Box<dyn ImplText>) -> String {
    let mut text_maker = TextMaker {
        text_imp: text_imp };
    text_maker.add_line("line 1");
    text_maker.add_line("line 2");
    return text_maker.get_text();
}

pub(crate) fn test() {

    let text = fill_text_builder(
        Box::new(TextBuilder{ rows: String::new() }));
    //text: line 1
    //      line 2
    println!("text:\n{text}");
    let html = fill_text_builder(
        Box::new(HtmlBuilder{ rows: String::new() }));
    //text: line 1
    //      line 2
    println!("html:\n{html}");

}