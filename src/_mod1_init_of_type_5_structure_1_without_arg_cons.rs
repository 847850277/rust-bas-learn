#[derive(Debug)]
#[allow(dead_code)]
struct Size {
    width: i32,
    height: i32
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    top: i32,
    left: i32
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    size: Size,
    point: Point
}
pub fn test(){


    let rect = Rectangle {
        size: Size { width: 10, height: 10 },
        point: Point { top: 5, left: 5 }
    };

    println!("rect is {:?}", rect);

}