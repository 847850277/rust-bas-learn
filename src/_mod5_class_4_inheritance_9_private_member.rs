mod prime {
    pub struct Shape {
        //private field
        uid: i32
    }

    impl Shape {
        pub fn new() -> Shape {
            return Shape { uid: 1 };
        }
        pub fn get_uid(self) -> i32 {
            return self.uid;
        }
    }
}
pub(crate) fn test() {

    use prime::*;
    let shape = Shape::new();
    //Error: field is private
    //println!("{}", shape.uid);
    println!("{}", shape.get_uid());

}