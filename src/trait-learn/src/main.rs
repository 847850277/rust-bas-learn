pub mod t_1;
mod t_2;
mod t_3;

fn main() {

    t_2::test();
    println!("-----------------");
    t_2::static_dispatch();
    println!("-----------------");
    t_2::dynamic_dispatch();
    println!("-----------------");
    t_3::test();

}


