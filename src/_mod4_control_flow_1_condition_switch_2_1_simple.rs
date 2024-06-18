use rustils::random::in_range;

pub(crate) fn test() {
    let mut monitor_inch_size = 15;

    for i in 1..10 {
        let str = get_size(&mut monitor_inch_size);
        println!("str is '{}'", str);
        monitor_inch_size += i;
    }
}

fn get_size(monitor_inch_size: &mut i32) -> &'static str {
    match monitor_inch_size {
        15 => "too small",
        16..=18 => "good for the past decade",
        19..=23 => "for office work",
        24..=27 => "great choice",
        _ => "",
    }
}