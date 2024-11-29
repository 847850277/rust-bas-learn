use starknet::core::types::FieldElement;

pub(crate) fn test() {
    let collections = vec![
        FieldElement::from_hex_be("0124").unwrap(),
        FieldElement::from_hex_be("123").unwrap(),
    ];
    // iter collection and then FieldElement.inner to string
    let mut body = String::new();
    for item in collections.iter() {
        // println!("{:?}", item);
        body.push_str(item.to_string().as_str());
        body.push(',');
        body.push('\n');
    }
    println!("{:?}", body);
    //
}
