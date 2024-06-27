use std::thread;
use std::time::Duration;
use futures::executor;
pub(crate) fn test() {
    let calc_and_print = async {
        let result = add(2, 9).await;
        println!("result is {result}");
    };
    executor::block_on(calc_and_print);
    println!("main thread!");
}

//crate futures = "0.3.24"
async fn add(a: i32, b: i32) -> i32 {
    thread::sleep(Duration::from_millis(1000));
    return a + b;
}