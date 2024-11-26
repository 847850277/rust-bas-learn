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