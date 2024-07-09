use std::mem::replace;
struct Settings<'a> {
    port: i32,
    host: &'a str
}
struct Peripherals<'a> {
    settings: Option<Settings<'a>>,
}
impl Peripherals<'_> {
    fn take_setting(&mut self) -> Settings {
        let p = replace(&mut self.settings, None);
        p.unwrap()
    }
}
static mut PERIPHERALS: Peripherals = Peripherals {
    settings: Some(Settings {
        port: 0,
        host: "",
    }),
};

pub(crate) fn test() {


    let mut settings1 = unsafe { PERIPHERALS.take_setting() };
    settings1.host = "192.168.100.1";
    settings1.port = 33;
    println!("port is {:#?}", settings1.port);
    println!("host is {:#?}", settings1.host);

    //This structure allows us to obtain a single instance of our peripheral. If we try to call take_serial() more than once, our code will panic!

}