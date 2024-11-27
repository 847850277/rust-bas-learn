use std::fmt::Display;

// trait 的约束条件
trait Animal{
    fn make_sound(&self) -> &'static str;
}

trait Food{}

struct Dog;

impl Animal for Dog{
    fn make_sound(&self) -> &'static str {
        "Woof!"
    }
}

struct Cat;

impl Animal for Cat{
    fn make_sound(&self) -> &'static str {
        "Meow!"
    }
}

struct Pig;

impl Animal for Pig{
    fn make_sound(&self) -> &'static str {
        "Woof!"
    }
}

impl Food for Pig {

}


impl Display for Pig{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pig")
    }
}

// trait 作为约束时有三种写法

// 参数必须实现那些接口写法1
fn get_weight<T: Animal + Food + Display>(x: T){

    // do sth
    println!("{}", x);


}

// 参数必须实现那些接口写法2
fn get_weight1(x: impl Animal + Food + Display){
    println!("{}", x);

    // do sth
}

// 参数必须实现那些接口写法3
fn get_weight2<T>(x: T) where T: Animal + Food + Display{

    println!("{}", x);
    // do sth
}

pub(crate) fn test(){
    let d = Dog;
    let c = Cat;
    let p = Pig;

    //get_weight(d);
    //get_weight(c);
    get_weight(p);

    let p1 = Pig;
    get_weight1(p1);

    let p2 = Pig;
    get_weight2(p2);
}

// 静态分发和动态分发

// trait 对象通过指针来创建，如 & 或 Box<T>(一种智能指针，可以把数据存放到堆上)：&dyn Trait or Box<dyn Trait>
// Box是Rust中唯一可以把数据强制分配到堆上的类型

// 静态分发:在编译期通过具体类型实例直接调用方法,编译期单态化

// 动态分发:在运行期通过指针调用方法,运行期多态化

// 静态分发
pub(crate) fn static_dispatch(){
    let d = Dog;
    let c = Cat;
    let p = Pig;

    // 静态分发
    // 编译期单态化
    // 编译期就知道调用的是哪个方法
    // 编译期就知道调用的是哪个方法
    // 编译期就知道调用的是哪个方法
    println!("{}", d.make_sound());
    println!("{}", c.make_sound());
    println!("{}", p.make_sound());
}

// 动态分发
pub(crate) fn dynamic_dispatch(){
    let d = Dog;
    let c = Cat;
    let p = Pig;

    // 动态分发
    // 运行期多态化
    // 运行期才知道调用的是哪个方法
    // 运行期才知道调用的是哪个方法
    // 运行期才知道调用的是哪个方法
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(d), Box::new(c), Box::new(p)];

    for animal in animals{
        println!("{}", animal.make_sound());
    }
}