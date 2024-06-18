struct Counter {
    count: i32
}

impl Counter {
    fn inc_by(&mut self, value: i32) {
        self.count += value;
    }
    fn inc_by_amount(&mut self, value: i32, amount: i32) {
        self.count += value * amount;
    }
}

pub(crate) fn test() {


    let mut counter = Counter {count: 0};
    counter.inc_by(1);
    //counter.count is 1
    println!("count is {}", counter.count);
    counter.inc_by_amount(2, 5);
    //counter.count is 11
    println!("count is {}", counter.count);

}