struct Calc {}
impl Calc {
    fn get_sum(self, sum: &mut i32, n1: i32, n2: i32) {
        *sum = n1 + n2;
    }
}

pub(crate) fn test() {
    let calc = Calc {};
    let mut sum: i32 = 0;
    calc.get_sum(&mut sum, 5, 3);
    //sum is 8
    println!("sum is {sum}");
}
