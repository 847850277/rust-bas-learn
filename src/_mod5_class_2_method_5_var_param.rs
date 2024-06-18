struct Log<'a> {
    last_data: &'a str
}
impl<'a> Log<'a> {
    fn print5(&mut self, mut data: &'a str) {
        self.last_data = data;
        if data.chars().count() > 5 {
            data = &data[0..5];
        }
        println!("{}", data);
    }
}

pub(crate) fn test() {

    let mut log = Log{ last_data: ""};
    log.print5("12345679");
//printed "12345"

}