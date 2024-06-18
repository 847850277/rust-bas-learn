struct Circle {
    radius: i32
}
impl Circle {
    fn area(&self) -> f32 {
        return std::f32::consts::PI *
            (self.radius.pow(2) as f32);
    }
}

pub(crate) fn test() {

    let circle = Circle {radius: 2};
    //circle.area is 12.566371
    print!("circle.area is {}", circle.area());

}