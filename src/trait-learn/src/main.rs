pub mod t_1;
mod t_2;
mod t_3;
mod t_4;
mod smart_point_1;
mod smart_point_2;
mod smart_point_3;

fn main() {

    t_2::test();
    println!("-----------------");
    t_2::static_dispatch();
    println!("-----------------");
    t_2::dynamic_dispatch();
    println!("-----------------");
    t_3::test();
    println!("-----------------");
    t_4::test();
    println!("-----------------");
    smart_point_1::test();
    println!("-----------------");
    smart_point_2::test();
    println!("-----------------");
    smart_point_3::test();
    println!("-----------------");
    smart_point_3::test_2();
    println!("-----------------");
    smart_point_3::test_3();
    println!("-----------------");
    smart_point_3::test_4();
}


