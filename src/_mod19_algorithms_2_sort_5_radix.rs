use chrono::Duration;
// Time Complexity O(nk)
// Space Complexity O(n+k)
fn list_to_buckets(items: &mut [i32],
                   base: i32, i: u32) -> Vec<Vec<i32>>
{
    let mut buckets = vec![Vec::<i32>::new(); base as usize];
    let p_base = base.pow(i);
    for i in 0..items.len() {
        let x = items[i];
        let digit = ((x / p_base) % base) as usize;
        buckets[digit].push(x);
    }
    return buckets;
}
fn buckets_to_list(buckets: Vec<Vec<i32>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for mut bucket in buckets {
        result.append(&mut bucket);
    }
    return result;
}
fn sort(items: &mut [i32]) {
    let max = *items.iter().max().unwrap();
    let mut i: u32 = 0;
    let base: i32 = 10;
    while base.pow(i) <= max {
        let result = buckets_to_list(list_to_buckets(items, base, i));
        items.copy_from_slice(&result);
        i += 1;
    }
}

pub(crate) fn test() {
    let mut nums = [ 4, 1, 5, 3, 2  ];
    sort(&mut nums);
    // sortItems is {1, 2, 3, 4, 5}
    println!("nums is {:?}", nums);
    // *** simplified speed test ***
    let mut items = [0i32; 200];
    for i in 0..items.len() {
        items[i] = i as i32;
    }
    let tmp = items[5];
    items[5] = items[6];
    items[6] = tmp;
    let count = 1000;
    let start = chrono::offset::Utc::now();
    for _ in 1..count {
        sort(&mut items);
    }
    let now = chrono::offset::Utc::now();
    let delta: Duration = now - start;
    println!("milliseconds is {}",
             delta.num_milliseconds());
    println!("items is {:?}", items);
    // about 171 milliseconds
}