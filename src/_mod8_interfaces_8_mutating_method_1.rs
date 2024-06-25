struct Mode {
    value: u8
}
trait Chain {
    fn next(&mut self);
}
impl Chain for Mode {
    fn next(&mut self) {
        self.value = (self.value + 1) % 4;
    }
}
pub(crate) fn test() {

    let mut mode = Mode{ value: 2 };
    mode.next();
    println!("mode is {:?}", mode.value);
    mode.next();
    //mode is 0
    println!("mode is {:?}", mode.value);

}