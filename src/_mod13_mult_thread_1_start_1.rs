use std::thread;
use std::time::Duration;

pub(crate) fn test() {

    let handle = thread::spawn(|| {
        let result = add(3, 5);
        println!("result: {}", result);
    });
    println!("main thread");
    handle.join().unwrap();
    //output:
    //main thread
    //result: 8

}

fn add(a: i32, b: i32) -> i32 {
    thread::sleep(Duration::from_millis(1000));
    return a + b;
}