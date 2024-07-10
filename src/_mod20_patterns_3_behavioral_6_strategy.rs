trait Strategy {
    fn do_operation(&self, a: i32, b:i32) -> i32;
}
//ConcreteStrategy
struct AddStrategy {}
impl Strategy for AddStrategy {
    fn do_operation(&self, a: i32, b:i32) -> i32 {
        return a + b;
    }
}
//ConcreteStrategy
struct SubstractStrategy {}
impl Strategy for SubstractStrategy {
    fn do_operation(&self, a: i32, b:i32) -> i32 {
        return a - b;
    }
}
//Context
struct Calc {
    strategy: Option<Box<dyn Strategy>>
}
impl Calc {
    fn execute(&self, a: i32, b:i32) -> i32 {
        if let Some(strategy) = &self.strategy {
            return strategy.do_operation(a, b);
        }
        return 0;
    }
    fn set_strategy(&mut self, strategy: Box<dyn Strategy>) {
        self.strategy = Some(strategy);
    }
}

pub(crate) fn test() {

    let mut calc = Calc{ strategy: None };
    let result1 = calc.execute(5, 3);
    //result1 is 0
    calc.set_strategy(Box::new(AddStrategy{}));
    let result2 = calc.execute(5, 3);
    //result2 is 8
    calc.set_strategy(Box::new(SubstractStrategy{}));
    let result3 = calc.execute(5, 3);
    //result3 is 2
    println!("result1 is {result1}");
    println!("result2 is {result2}");
    println!("result3 is {result3}");

}