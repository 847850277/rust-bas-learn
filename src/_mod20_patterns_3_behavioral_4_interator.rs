//Iterator
trait IntIterator  {
    fn first(&mut self);
    fn next(&mut self);
    fn is_done(&self) -> bool;
    fn get_current(&self) -> i32;
}
//ConcreteIterator
struct Iterator {
    index: usize,
    parrent: PrimeNumbers
}
impl IntIterator for Iterator {
    fn first(&mut self) {
        self.index = 0;
    }
    fn next(&mut self) {
        self.index += 1;
    }
    fn is_done(&self) -> bool {
        return self.index >= self.parrent.numbers.len();
    }
    fn get_current(&self) -> i32 {
        return self.parrent.numbers[self.index];
    }
}
//ConcreteAggregate
struct  PrimeNumbers {
    numbers: Vec<i32>
}
impl PrimeNumbers {
    fn get_iterator(self) -> Box<dyn IntIterator> {
        return Box::new(Iterator {
            index: 0,
            parrent: self
        });
    }
}
pub(crate) fn  test() {

    //Client
    let numbers = PrimeNumbers {
        numbers: vec![2, 3, 5, 7, 11]
    };
    let mut iterator = numbers.get_iterator();
    let mut sum = 0;
    iterator.first();
    while !iterator.is_done() {
        sum += iterator.get_current();
        iterator.next();
    }
    //sum is 28
    println!("sum is {sum}");

}