//AbstractBuilder
trait ITextBuilder {
    fn add_text(&mut self, value: &str);
    fn add_new_line(&mut self, value: &str);
    fn get_text(&self) -> String;
}
//ConcreteBuilder 1
struct TextBuilder {
    text: String,
}
impl ITextBuilder for TextBuilder {
    fn add_text(&mut self, value: &str) {
        self.text.push_str(value);
    }
    fn add_new_line(&mut self, value: &str) {
        self.text.push_str("\n");
        self.add_text(value);
    }
    fn get_text(&self) -> String {
        return self.text.clone();
    }
}
//ConcreteBuilder 2
struct HtmlBuilder {
    html: String,
}
impl ITextBuilder for HtmlBuilder {
    fn add_text(&mut self, value: &str) {
        self.html.push_str("<span>");
        self.html.push_str(value);
        self.html.push_str("</span>");
    }
    fn add_new_line(&mut self, value: &str) {
        self.html.push_str("<br/>\n");
        self.add_text(value);
    }
    fn get_text(&self) -> String {
        return self.html.clone();
    }
}
//Director
struct TextMaker {}
impl TextMaker {
    fn make_text(mut text_builder: Box<dyn ITextBuilder>) {
        text_builder.add_text("line 1");
        text_builder.add_new_line("line 2");
        let text = text_builder.get_text();
        println!("{text}");
    }
}
pub(crate) fn test() {
    //Client
    let text_builder = Box::new(TextBuilder {
        text: String::new(),
    });
    TextMaker::make_text(text_builder);
    //printed: line 1
    //         line 2
    let html_builder = Box::new(HtmlBuilder {
        html: String::new(),
    });
    TextMaker::make_text(html_builder);
    //printed: <span>line 1</span><br/>
    //         <span>line 2</span>
}
