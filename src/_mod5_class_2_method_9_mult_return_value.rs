struct ArrayAssistant<'a> {
    data: &'a [i32]
}
impl ArrayAssistant<'_>
{
    fn get_first_last(&self) -> (i32, i32) {
        let mut first = -1;
        let mut last = -1;
        let len = self.data.len();
        if len > 0 {
            first = self.data[0];
            last = self.data[len - 1];
        }
        return (first, last);
    }
}

pub(crate) fn test() {

    let ar = [2, 3, 5];
    let assistant  = ArrayAssistant{ data: &ar };
    let result = assistant.get_first_last();
    //result.first is 2
    //result.last is 5
    println!("first is {:?}", result.0);
    println!("last is {:?}", result.1);
}