

#[allow(unused_macros)]
macro_rules! say_goodby {
    ($a: expr) => {
        say_goodby($a);
    };
    () => {
        say_goodby("Goodby!");
    };
}

//prints "see you"
fn say_goodby(message: &str) {
    println!("{}", message);
}

pub(crate) fn test() {
    say_goodby!();
    //prints "Goodby!"
    say_goodby!("see you");
}