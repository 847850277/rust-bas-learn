use std::fs;
use serde_derive::Deserialize;

//serde_derive = "1.0"
//serde = "1.0"
//serde-xml-rs = "0.3.1"
//XML example:
//<Lines>
//    <Line Id="1">one</Line>
//    <Line Id="2">two</Line>
//</Lines>
#[derive(Deserialize, Debug)]
struct Lines {
    #[serde(rename="Line")]
    lines: Vec<Line>,
}
#[derive(Deserialize, Debug)]
struct Line {
    #[serde(rename="Id")]
    id: u8,
    #[serde(rename="$value")]
    content: String,
}

pub(crate) fn test() {
    main().unwrap();
}

fn main() -> std::io::Result<()> {
    let path = "/Users/zhengpeng/Work/Source/Code/rust-code/rust-bas-learn/test.xml";
    let text = fs::read_to_string(path)?;
    let lines: Lines = serde_xml_rs::from_str(
        &text).unwrap();
    for line in lines.lines {
        println!("value: {:?}, id: {}",
                 line.content, line.id);
    }
    Ok(())
}