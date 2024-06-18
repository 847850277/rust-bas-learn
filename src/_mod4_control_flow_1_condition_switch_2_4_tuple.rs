pub(crate) fn test() {
    for i in 1..10 {
        let point = get_point(i);
        match point {
            (0, 0) => println!("(0, 0) point"),
            (_, 1) => println!("({}, 1) point", point.0),
            (1, _) => println!("(1, {}) point", point.1),
            (2..=5, _) => println!("({}, {}) point",
                                 point.0, point.1),
            (_, _) => println!("other point"),
        }
    }

}

fn get_point(i: i32) -> (i32,i32) {
    if(i % 2 == 0){
        return (0, 0);
    }
    if(i % 3 == 0){
        return (9, 1);
    }
    if(i % 4 == 0){
        return (1, 5);
    }
    if(i % 5 == 0){
        return (2, 5);
    }
    return (5,5);
}