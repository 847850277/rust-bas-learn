struct Calc {}
impl Calc {
    fn get_avg(values: &[i32]) -> f32 {
        if values.len() == 0 {
            return 0.0;
        }
        let mut sum: i32 = 0;
        for i in values {
            sum += i;
        }
        return (sum as f32) / (values.len() as f32);
    }
}

pub(crate) fn test() {
    let array = [1, 2, 3, 4];
    let avg = Calc::get_avg(&array);
    //avg is 2.5
    println!("avg is {avg}");
}
