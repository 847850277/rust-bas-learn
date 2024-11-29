use chrono::*;
pub(crate) fn test() {
    let now = chrono::offset::Utc::now();
    let yesterday = now - Duration::days(1);

    let are_equal = now == yesterday;
    //areEqual is false

    let are_later = now > yesterday;
    //areLater is true

    let are_earlier = now < yesterday;
    //areEarlier is false

    println!("now       is {now}");
    println!("yesterday is {yesterday}");
    println!("are_equal is {are_equal}");
    println!("are_later is {are_later}");
    println!("are_earlier is {are_earlier}");
}
