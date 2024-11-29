use chrono::Duration;
fn search(arr: &[i32], x: i32) -> Option<usize> {
    let mut i = 0;
    let count = arr.len();
    while i < count {
        if arr[i] == x {
            return Some(i);
        }
        i += 1;
    }
    return None;
}

pub(crate) fn test() {
    let nums = [2, 3, 5, 7, 11, 13, 17];
    println!("{:?}", search(&nums, 1));
    //print -1
    println!("{:?}", search(&nums, 7));
    //print 3
    println!("{:?}", search(&nums, 19));
    //print -1
    // *** simplified speed test ***
    let mut items = [0i32; 1_000_0];
    for i in 0..items.len() {
        items[i] = i as i32;
    }
    let count = 100;
    let start = chrono::offset::Utc::now();
    let mut result = None;
    for _ in 1..count {
        result = search(&items, 777_777);
    }
    let now = chrono::offset::Utc::now();
    let delta: Duration = now - start;
    println!("result is {:?}", result);
    println!("{}", delta.num_milliseconds());
    // about 389 milliseconds
}
