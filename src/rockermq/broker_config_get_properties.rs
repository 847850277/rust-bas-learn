use std::any::Any;
use std::collections::HashMap;

pub(crate) fn test() {
    //todo!()
    let map = get_properties();
    println!("{:?}", map);
    // iter map
    for (key, value) in map.iter() {
        println!("key: {:?}, value: {:?}", key, value);
        value.downcast_ref::<i32>().map(|v| {
            println!("i32: {:?}", v);
        });
        value.downcast_ref::<String>().map(|v| {
            println!("String: {:?}", v);
        });
        value.downcast_ref::<bool>().map(|v| {
            println!("bool: {:?}", v);
        });
    }
}

pub fn get_properties() -> HashMap<String, Box<dyn Any>> {
    let mut properties = HashMap::new();
    properties.insert("brokerName".to_string(), Box::new(1) as Box<dyn Any>);
    properties.insert(
        "broker_identity.brokerId".to_string(),
        Box::new("2".to_string()) as Box<dyn Any>,
    );
    properties.insert(
        "broker_identity.brokerClusterName".to_string(),
        Box::new(true) as Box<dyn Any>,
    );
    properties
}

pub fn get_properties_1() -> HashMap<String, String> {
    let mut properties = HashMap::new();
    // properties.insert("brokerName".to_string(), 1);
    properties.insert("broker_identity.brokerId".to_string(), String::new());
    // properties.insert("broker_identity.brokerClusterName".to_string(), true);
    properties
}

pub(crate) fn test_1() {
    let map = get_properties_1();
    println!("{:?}", map);
}
