// 引用
// 在更广泛的意义上，指针代表指向某个变量所对应的数据内存地址。所以引用也是一种指针，Rust中的引用也不例外。引用的获取和使用非常简单：如下

//在Rust中，引用的使用总是安全的，因为引用在Rust中受所有权机制约束。因此你可以在所有权和借用检查规则之下比较灵活的使用引用
pub(crate) fn test(){
    let x = 10;
    let mut y : i64 = 200;
    #[derive(Debug)]
    struct A(i32);
    let a = A(100);
    // 使用&获取可变引用和不可变引用
    let x_pointer = &x;
    let y_pointer = &mut y;
    let a_pointer = &a;

    println!("x_pointer is {:?}", x_pointer);//打印值
    println!("x_pointer is {:p}", x_pointer);//打印地址 占位符为{:p}

    //y = 300;//编译会报错
    println!("y_pointer is {:?}", y_pointer);//打印值

    // 解引用后修改
    y = *y_pointer + 100;
    println!("y_pointer is {:?}", y);//打印值,本代码结束后，可变引用才释放
    println!("a_pointer is {:?}", a_pointer);//打印值

}
