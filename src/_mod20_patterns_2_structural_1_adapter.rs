//Target
trait Text {
    //Request()
    fn get_text(&self) -> String;
}
//Adaptee
struct StringList<'a> {
    rows: Vec<&'a str>,
}
impl<'a> StringList<'a> {
    //SpecificRequest()
    fn get_string(&self) -> String {
        return self.rows.join("\n");
    }
    fn add(&mut self, value: &'a str) {
        self.rows.push(value);
    }
}
//Adapter
struct TextAdapter<'a> {
    row_list: StringList<'a>,
}
impl Text for TextAdapter<'_> {
    //Request()
    fn get_text(&self) -> String {
        return self.row_list.get_string();
    }
}
fn get_text_adapter<'a>() -> TextAdapter<'a> {
    let mut row_list = StringList { rows: vec![] };
    row_list.add("line 1");
    row_list.add("line 2");
    let adapter = TextAdapter { row_list: row_list };
    return adapter;
}

pub(crate) fn test() {
    //Client
    let adapter = get_text_adapter();
    let text = adapter.get_text();
    //text: line 1
    //      line 2
    println!("text:\n{text}");
}
