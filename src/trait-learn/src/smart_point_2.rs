// 裸指针
// Rust中的裸指针‌是一种特殊的指针类型，用于提供对内存的直接操作能力，但不受Rust的所有权和借用规则的保护。裸指针主要有两种类型：*const T和*mut T，分别表示不可变和可变的裸指针‌
// 在Rust中，裸指针在生成的时候是安全的，但是对裸指针的解引用是不安全的，非要解引用的话，需要在 unsafe {} 语法块中。来看下面的例子，另外，这涉及Unsafe Rust中的内容，我们暂不做深入，这里介绍的意思是涵盖一下Rust中的指针都有哪些，所以了解即可。关于Unsafe Rust，我们后面再大讲特讲

pub(crate) fn test(){

    let x = 100;
    let mut y: i64 = 200;
    struct B(i32);
    let a = B(100);

    // 裸指针 是一个普通的指针，不受Rust所有权和借用检查规则约束
    // 裸指针 是使用 as *const T 或 as *mut T 语法转化而来的
    let x_raw_pointer = &x as *const i32;
    let y_raw_pointer = &mut y as *mut i64;
    let a_raw_pointer = &a as *const B;

    println!("x_raw_pointer is {:?}", x_raw_pointer);//裸指针打印时，不会被解引用，而是会打印地址

    unsafe {
        y = *y_raw_pointer + 100; // 裸指针解引用需要使用unsafe语法块,这里的解引用是安全的
        let z_raw_pointer = y_raw_pointer as *const i64; // 第二次生成了可变裸指针，unsafe块绕过了可变借用次数的规则，是不是有点危险？
        println!("z_raw_pointer is {:?}", *z_raw_pointer);
        y = *z_raw_pointer + 100;
        println!("y is {:?}", *y_raw_pointer);
    }
    println!("a_raw_pointer is {:?}", a_raw_pointer);
    println!("y is {:?}", y);

}

