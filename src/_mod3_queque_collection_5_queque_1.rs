pub(crate) fn test() {
    let mut int_stack = vec![];
    int_stack.push(1);
    int_stack.push(3);
    int_stack.push(5);
    let top = int_stack[int_stack.len() - 1];
    //top is 5
    let first = int_stack.pop();
    //first is Some(5)
    let second = int_stack.pop();
    //second is Some(3)
    let third = int_stack.pop();
    //third is Some(1)
    println!("top is {:?}", top);
    println!("first is {:?}", first);
    println!("second is {:?}", second);
    println!("third is {:?}", third);
}
