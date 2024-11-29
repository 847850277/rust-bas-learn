use std::thread;
pub(crate) fn test() {
    let numbers = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("numbers is {:?}", numbers);
    });
    //println!("numbers is {:?}", numbers);<-Error
    handle.join().unwrap();

    //Use the "move" keyword to indicate that the variable is passed to the secondary thread and will not be used in the main thread.
}
