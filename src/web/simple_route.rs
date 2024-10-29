use std::collections::HashMap;

type Handler = fn();

fn handler1() {
    println!("Handler 1");
}

fn handler2() {
    println!("Handler 2");
}

pub(crate) fn main() {
    let mut map: HashMap<String, Handler> = HashMap::new();

    map.insert("/route1".to_string(), handler1 as Handler);
    map.insert("/route2".to_string(), handler2 as Handler);

    // Now you can call the function associated with a route like this:
    // loop for map and print
    let routes = map.keys();
    for route in routes {
        if let Some(handler) = map.get(route) {
            handler();
        } else {
            println!("No handler found for route {}", route);
        }
    }
}