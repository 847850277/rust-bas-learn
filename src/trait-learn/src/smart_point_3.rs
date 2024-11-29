// 智能指针
// 智能指针与引用的区别主要表现在它“拥有”数据，而不只是“借用”数据，来看个例子


use std::ops::Deref;

pub fn test(){

    // Vec 和 String 都是智能指针
    let vec = vec![1, 2, 3,4];
    let s = "Rust".to_string();
    let num = Box::new(5);
    let v1 = vec; // 这里发生了所有权转移，现在数据的所有权属于 v1，vec 不能再使用
    // println!("{:?}", vec); // 这里会报错，因为 vec 的所有权已经转移给了 v1
    let v = vec![1, 2, 3, 4];
    let v = &v; // 这里没有发生所有权转移，v 仍然可以使用
    println!("{:?}", v);

}

// Box<T> 也是智能指针,并且它可以把数据强制放到堆上（也是Rust中唯一可以把数据强制放到堆上的方法）
// 常见的智能指针
// Box<T>：用于在堆上分配内存
// Rc<T>：引用计数智能指针，用于同一份数据有多个所有者的情况
// Arc<T>：原子引用计数智能指针，用于多线程环境下的 Rc<T>
// String：字符串类型
// Vec<T>：动态数组类型

// 那智能指针和 trait又有什么关系呢？原来在Rust中，智能指针的“智能”通过 trait来实现，与智能指针相关的trait有Drop trait 和Deref trait
// Drop trait：当智能指针超出作用域时，会调用Drop trait的drop方法，这个方法可以用来释放资源
// 也可以说只要实现了 Deref 或者 Drop trait的类型，都算智能指针，

pub fn test_2(){

    #[derive(Debug)]
    struct User{
        name: String,
        age: u8
    }

    let user = User{
        name: "Rust".to_string(),
        age: 5
    };

    drop(user);

    //println!("{:?}", user); // 这里会报错，因为 user 的所有权已经转移给了 drop 方法

}

pub fn test_3(){

    #[derive(Debug)]
    struct User{
        name: String,
        age: u8
    }

    impl Drop for User{
        fn drop(&mut self){
            println!("Dropping User: {:?}", self);
        }
    }

    let user = User{
        name: "Rust".to_string(),
        age: 5
    };

    println!("user: {:?}", user);

    // 这里会调用 drop 方法
    // 但是这里并不是手动调用 drop 方法，而是当 user 超出作用域时，Rust 会自动调用 drop 方法
}


// Deref trait：智能指针可以通过实现 Deref trait 来重载解引用运算符 *，这样智能指针就可以像引用一样使用
// 例如 Box<T> 实现了 Deref trait，所以 Box<T> 可以像引用一样使用

pub fn test_4(){

    #[derive(Debug)]
    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            &self.0
        }
    }

    impl <T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let m = MyBox::new("Rust");
    println!("{:?}", m);
    let ref_my_box = *m;// 实现了 Deref trait，所以可以使用*直接解引用
    println!("{:?}", ref_my_box);

}


// https://zhuanlan.zhihu.com/p/614970269