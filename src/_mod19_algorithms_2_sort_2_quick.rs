use chrono::Duration;
// Time Complexity from O(n log(n)) to O(n^2)
// Space Complexity O(log(n))
fn do_sort(items: &mut [i32], fst: usize, lst: usize) {
    if fst >= lst {
        return;
    }
    let mut i = fst;
    let mut j = lst;
    let x = items[(fst + lst) / 2];
    while i < j {
        while items[i] < x {
            i += 1
        }
        while items[j] > x {
            j -= 1
        }
        if i <= j {
            let tmp = items[i];
            items[i] = items[j];
            items[j] = tmp;
            i += 1;
            if j > 0 {
                j -= 1
            }
        }
    }
    do_sort(items, fst, j);
    do_sort(items, i, lst);
}

pub(crate) fn test() {
    fn sort(items: &mut [i32]) {
        do_sort(items, 0, items.len() - 1);
    }
    let mut nums = [4, 1, 5, 3, 2];
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
    println!("milliseconds is {}", delta.num_milliseconds());
    println!("items is {:?}", items);
    // about 7 milliseconds
}
