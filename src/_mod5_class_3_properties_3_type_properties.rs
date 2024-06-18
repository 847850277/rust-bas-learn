use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Debug)]
struct Lesson {}
impl Lesson {
    fn new() -> Lesson {
        static COUNT: AtomicUsize = AtomicUsize::new(1);
        println!("Lessons count is {}",
                 COUNT.fetch_add(1, Ordering::Relaxed));
        return Lesson {};
    }
}

pub(crate) fn test() {

    let lesson1 = Lesson::new();
    let lesson2 = Lesson::new();
    println!("lesson1 is {:?}", lesson1);
    println!("lesson2 is {:?}", lesson2);

}