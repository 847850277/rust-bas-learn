//There are no classes in Rust

use std::collections::HashMap;
use num::cast::ToPrimitive;

struct Size<T> {
    width: T,
    height: T
}
impl<T: std::fmt::Debug> Size<T> {
    fn new(width: T, height: T) -> Size<T> {
        return Size {
            width: width, height: height
        };
    }
    fn as_text(&self) -> String {
        return format!("[{:?}, {:?}]", self.width, self.height);
    }
}


struct Response<T>{
    body: T,
}

// debug
impl <T: std::fmt::Debug> Response<T>{
    fn new(body: T) -> Response<T>{
        Response{
            body
        }
    }

    fn as_text(&self) -> String {
        return format!("{:?}", self.body);
    }

}

impl<T: ToPrimitive> Response<T> {
    fn as_int(&self) -> Option<i32> {
        self.body.to_i32()
    }
}

impl <T: Default> Response<T>{
    fn new_empty() -> Response<T>{
        Response{
            body: Default::default()
        }
    }
}

// new
impl <T> Response<T>{
    fn set_body(&mut self, body: T) {
        self.body = body;
    }

    fn get_body(&self) -> &T {
        return &self.body;
    }
}


pub(crate) fn test() {

    let size_int = Size::new(5, 8);
    let text_int = size_int.as_text();
    //text_int is "[5; 8]"
    let size_float = Size::new(3.7, 1.58);
    let text_float = size_float.as_text();
    //text_float is "[3.7; 1.58]"
    println!("text_int is {text_int}");
    println!("text_float is {text_float}");

}


pub fn test1() {

    //str test
    let response = Response::new("Hello".to_string());
    let text = response.as_text();
    //text is "Hello"
    println!("text is {text}");
    let mut response = Response::new_empty();
    response.set_body("World".to_string());
    let text = response.as_text();
    //text is "World"
    println!("text is {text}");
    let body = response.get_body();
    //body is "World"
    println!("body is {body}");

    // int test
    let response = Response::new(5);
    let text = response.as_text();
    let x = response.get_body();
    //text is "5"
    println!("text is {text}");
    let mut response = Response::new_empty();
    response.set_body(8);
    let text = response.as_int();
    //text is "8"
    println!("text is {:?}", text);


}