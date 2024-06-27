use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn add(a: i32, b: i32) -> i32 {
    thread::sleep(Duration::from_millis(1000));
    return a + b;
}

pub(crate) fn test() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let result = add(3, 5);
        tx.send(result).unwrap();
    });
    let result = rx.recv().unwrap();
    println!("result is {result}");

}