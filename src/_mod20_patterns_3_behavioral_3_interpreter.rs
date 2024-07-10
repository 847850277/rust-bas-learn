//AbstractExpression
trait Expression {
    fn interpret(&self, i: i32) -> bool;
}
//TerminalExpression
#[derive(Copy, Clone)]
struct DivExpression {
    divider: i32
}
impl Expression for DivExpression {
    fn interpret(&self, i: i32) -> bool {
        return i % self.divider == 0;
    }
}
//NonterminalExpression
struct OrExpression {
    exp1: Box<dyn Expression>,
    exp2: Box<dyn Expression>,
}
impl Expression for OrExpression {
    fn interpret(&self, i: i32) -> bool {
        return self.exp1.interpret(i) ||
            self.exp2.interpret(i);
    }
}
//NonterminalExpression
struct AndExpression {
    exp1: Box<dyn Expression>,
    exp2: Box<dyn Expression>,
}
impl Expression for AndExpression {
    fn interpret(&self, i: i32) -> bool {
        return self.exp1.interpret(i) &&
            self.exp2.interpret(i);
    }
}

pub(crate) fn test() {

    //Client
    let div_exp5 = DivExpression {
        divider: 5 };
    let div_exp7 = DivExpression {
        divider: 7 };
    let or_exp = OrExpression {
        exp1: Box::new(div_exp5),
        exp2: Box::new(div_exp7) };
    let and_exp = AndExpression {
        exp1: Box::new(div_exp5),
        exp2: Box::new(div_exp7) };
    //21 is divided by 7 or 5?
    let result1 = or_exp.interpret(21);
    //result1 is true
    println!("result1 is {result1}");
    //21 is divided by 7 and 5?
    let result2 = and_exp.interpret(21);
    //result2 is false
    println!("result2 is {result2}");

}