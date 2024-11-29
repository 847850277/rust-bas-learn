pub(crate) fn test() {
    print_some_data(true);
}

fn print_some_data(print_all: bool) {
    print_main_data();
    if !print_all {
        return;
    }
    print_other_data();
}
fn print_main_data() {
    println!("print_main_data");
}
fn print_other_data() {
    println!("print_other_data");
}
