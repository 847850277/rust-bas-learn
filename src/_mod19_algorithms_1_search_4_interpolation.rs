use chrono::Duration;
//works when the array is sorted
fn search(list: &[i32], x: i32) -> Option<usize>
{
    let mut low  = 0;
    let mut high  = list.len() - 1;
    while (list[low] < x) && (x < list[high])
    {
        let mid = low + ((x as usize - list[low] as usize)*(high - low))/(
            (list[high] - list[low]) as usize);
        if list[mid] < x {
            low = mid + 1;
        } else if list[mid] > x {
            high = mid - 1;
        } else {
            return Some(mid);
        }
    }
    if list[low] == x {
        return Some(low);
    }
    if list[high] == x {
        return Some(high);
    }
    return None;
}

pub(crate) fn test() {

    let nums = [ 2, 3, 5, 7, 11, 13, 17 ];
    println!("{:?}", search(&nums, 1));
    //print -1
    println!("{:?}", search(&nums, 7));
    //print 3
    println!("{:?}", search(&nums, 19));
    //print -1
    // *** simplified speed test ***
    let mut items = [0i32; 1_000_00];
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
    // about 0.004 milliseconds

}