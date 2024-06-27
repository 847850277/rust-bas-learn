use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn add(a: i32, b: i32) -> i32 {
    thread::sleep(Duration::from_secs(1));
    return a + b;
}
pub(crate) fn test() {

//first method
    thread::spawn(|| {
        let result = add(3, 5);
        println!("result is {result}");
    }).join().unwrap();
    println!("main thread");
    println!();
    //output:
    //main thread
    //result: 8
    //second method
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result1 = add(8, 7);
        tx.send(result1).unwrap();
        let result2 = add(80, 70);
        tx.send(result2).unwrap();
    });
    for result in rx {
        println!("result is {result}");
    }
    println!("main thread");

}