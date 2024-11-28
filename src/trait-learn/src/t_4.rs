// trait 所有权

pub(crate) fn test() {
    // 固定尺寸类型，赋值的时候会发生复制
    let owner_one = 32;
    let owner_two = owner_one;
    println!("owner_one: {}", owner_one);
    println!("owner_two: {}", owner_two);

    // 动态分配类型，赋值的时候会发生移动(所有权的转移)

    // String类型
    let owner_one = String::from("hello");
    let owner_two = owner_one;
    //println!("owner_one: {}", owner_one); // error: value borrowed here after move
    println!("owner_two: {}", owner_two);

    // 从trait的角度来讲，就是所有固定尺寸类型都是Copy的 和 Clone trait，而动态尺寸类型都没有实现Copy trait，但大多都实现了Clone trait
    // 并且编译器报错也会告诉你，哪些类型没有实现 Copy trait
    // 如果你想在堆上复制想像使用固定尺寸类型那样一样在堆上复制一份数据，你可以显式调用Clone trait中的 clone方法来实现这一点
    let v = vec![1, 2, 3];

    let v1 = v.clone();
    let v2 = v.clone();
    let v3 = v1.clone();

    // 新变量的地址和原变量的地址各不相同

    println!("{:p}", v.as_ptr()); // 0x6000009b0030
    println!("{:p}", v1.as_ptr()); // 0x6000009b0060
    println!("{:p}", v2.as_ptr()); // 0x6000009b0050
    println!("{:p}", v3.as_ptr()); // 0x6000009b0070

    // trait 实现与所有权
    // 在自定义 trait中的方法时，你可以根据需要选择要获取类型的不可变引用、可变引用或者所有权
    // 例如，如果你想要修改类型的值，你可以选择获取一个可变引用
    // 如果你想要获取类型的所有权，你可以选择获取类型的所有权
    // 如果你想要获取类型的不可变引用，你可以选择获取类型的不可变引用
    trait A{
        // 需要手动实现，获取所有权
        fn take_ownership(self);

        // 需要手动实现，获取不可变引用
        fn get(&self){
            println!("这个方法获取了类型的不可变引用")
        }

        // 需要手动实现，获取可变引用
        fn get_mut(&mut self){
            println!("这个方法获取了类型的可变引用")
        }
    }

    struct X{
    }

    impl A for X{
        fn take_ownership(self){
            println!("take ownership");
        }
    }

    let x = X{};
    x.take_ownership();
    // x.get(); 不能再使用x,因为上述方法已经获取了所有权
    let mut y = X{};
    y.get(); // 这个方法获取了类型的不可变引用
    y.get_mut(); // 这个方法获取了类型的可变引用

    // 特别说明：所有权机制和trait本质上是Rust中两个独立的概念，即使没有trait，所有权机制也是成立的（这也是我们在介绍所有权机制时为什么没有提及trait，因为不需要）
    // 但trait系统让所有权机制更加的显式化了，更好理解，也更好使用
}