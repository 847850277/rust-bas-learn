trait Text {
    fn as_text(&self) -> String {
        return "text".to_string();
    }
}
trait Html {
    fn as_html(&self) -> String {
        return "<span>html</span>"
            .to_string();
    }
}
struct TextHtml;
impl Text for TextHtml {}
impl Html for TextHtml {}

pub(crate) fn test() {

    let t_html = TextHtml{};
    let text = &t_html as &dyn Text;
    let s_text = text.as_text();
    //s_text is "text"
    let html = &t_html as &dyn Html;
    let s_html = html.as_html();
    //html is "<span>html</span>"
    println!("text is '{s_text}'");
    println!("html is '{s_html}'");

}