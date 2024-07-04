use std::fs::File;
use simple_xml_builder::XMLElement;
pub(crate) fn test() {

    //Crate simple-xml-builder = "1"
//XML example:
//<Lines>
//    <Line Id="1">one</Line>
//    <Line Id="2">two</Line>
//</Lines>
    fn main() -> std::io::Result<()> {
        let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/test.xml";
        let file = File::create(path)?;
        let mut lines = XMLElement::new("Lines");
        let mut line = XMLElement::new("Line");
        line.add_attribute("Id", "1");
        line.add_text("one");
        lines.add_child(line);
        let mut line = XMLElement::new("Line");
        line.add_attribute("Id", "2");
        line.add_text("two");
        lines.add_child(line);
        lines.write(file)?;
        print!("Successfully written!");
        Ok(())
    }
}