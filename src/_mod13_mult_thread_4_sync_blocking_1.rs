use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
pub(crate) fn test() {

    let show_number = move |number: i32| {
        thread::sleep(Duration::from_millis(100));
        print!("{number}, ");
    };
    let mut handles:Vec<JoinHandle<()>> = vec![];
    let barrier = Arc::new(Barrier::new(1));
    for i in 1..11 {
        thread::sleep(Duration::from_millis(1));
        let c:Arc<Barrier> = barrier.clone();
        handles.push(thread::spawn(move || {
            show_number(i);
            c.wait();
        }));
    }
    //result without c.wait: 8, 4, 9, 6, 1, 10, 7, 3, 2, 5
    //result with c.wait: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10
    // Wait for all threads to complete execution
    for handle in handles {
        handle.join().unwrap();
    }
    //A barrier will block n-1 threads which call wait() and then wake up all threads at once when the nth thread calls wait().

}