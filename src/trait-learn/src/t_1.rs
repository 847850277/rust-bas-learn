/// trait 声明

// 空trait
trait A{

}

// 有方法的trait

trait B{
    fn method(&self);

    fn method2(&self);
}

// 有关联类型的trait

trait C{
    type T;
    fn method(&self) -> Self::T;
}

// 有默认实现的trait

trait D{
    fn method1(&self){
        println!("method1");
    }

    fn consume_method(&mut self);
}

// 有自由方法（函数）的trait
trait E{

    //方法默认实现
    fn method1(&self){
        println!("method1");
    }

    fn method2(&self);

    //方法默认实现
    fn method3(){
        println!("freedom method");
    }

    fn method4(a: &str) -> &str;
}

//  trait继承

trait F: E{

}

// 继承多个trait
trait G: F + D + C + B + A{

}


// trait实现
struct Teacher;

impl Teacher {
    fn method1() {
        print!("这是类型的关联方法");
    }
}

fn test() {
    // 关联方法调用
    Teacher::method1();
}


impl A for Teacher {

}

impl B for Teacher {
    fn method(&self) {
        print!("")
    }
    fn method2(&self) {
        print!("")
    }
}

/*
fn test1() {
    let teacher = Teacher;
    teacher.method();
    teacher.method2();
}
*/

impl C for Teacher {
    type T = Teacher;
    fn method(&self) -> Self::T {
        let t = String::from("Teacher");
        Teacher
    }
}

impl D for Teacher {
    fn consume_method(&mut self) {
        println!("consume_method");
    }
}

/*
fn test2() {
    let mut teacher = Teacher;
    teacher.consume_method();
    teacher.consume_method();
}
 */

impl E for Teacher {
    fn method2(&self) {}
    fn method4(a: &str) -> &str {
        a
    }
}

/*
fn test3() {
    let teacher = Teacher;
    teacher.method2();
    E::method3();
    E::method4("a");
}
 */

struct Professor;

impl F for Teacher {

}

impl E for Professor {
    fn method2(&self) {
        todo!()
    }

    fn method4(a: &str) -> &str {
        todo!()
    }
}

impl F for Professor {

}

#[derive(Clone,Default)]
struct Student{
    name: String,
    age: u32,
}

/*
fn test4(){
    let student = Student::default();
    let student1 = student.clone();
}
 */