struct Man {
    age: i8
}

pub(crate) fn test() {
    let man = Man { age:18 };
    println!("age is {}", man.age);
}