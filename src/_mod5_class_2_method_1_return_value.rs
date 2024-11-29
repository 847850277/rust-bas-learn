struct Calc {
    last_sum: i32,
}
impl Calc {
    fn get_sum(&mut self, n1: i32, n2: i32) -> i32 {
        self.last_sum = n1 + n2;
        return self.last_sum;
    }
}

pub(crate) fn test() {
    let mut calc = Calc { last_sum: -1 };
    let sum1 = calc.get_sum(5, 3);
    //sum1 is 8
    let sum2 = calc.get_sum(2, 3);
    //sum2 is 5
    println!("sum1 = {sum1}");
    println!("sum2 = {sum2}");
    println!("calc.lastSum = {}", calc.last_sum);
}
