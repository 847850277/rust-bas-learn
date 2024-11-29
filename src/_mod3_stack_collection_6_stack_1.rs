pub(crate) fn test() {
    use std::collections::VecDeque;
    let mut int_queue = VecDeque::new();
    int_queue.push_back(1);
    int_queue.push_back(3);
    int_queue.push_back(5);
    let top = int_queue[0];
    //top is 1
    let first = int_queue.pop_front();
    //first is Some(1)
    let second = int_queue.pop_front();
    //second is Some(3)
    let third = int_queue.pop_front();
    //third is Some(5)
    println!("top is {:?}", top);
    println!("first is {:?}", first);
    println!("second is {:?}", second);
    println!("third is {:?}", third);
}
