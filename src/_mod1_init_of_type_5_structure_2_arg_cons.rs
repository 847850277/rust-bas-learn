#[derive(Debug)]
#[allow(dead_code)]
struct Size {
    width: i32,
    height: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    top: i32,
    left: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    size: Size,
    point: Point,
}

pub fn test() {
    impl Rectangle {
        fn new(width: i32, height: i32, top: i32, left: i32) -> Rectangle {
            return Rectangle {
                size: Size {
                    width: width,
                    height: height,
                },
                point: Point {
                    top: top,
                    left: left,
                },
            };
        }
    }

    let rect = Rectangle::new(10, 10, 5, 5);

    println!("rect is {:?}", rect);
}
