struct ScopeCall<F: FnMut()> {
    c: F,
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}
macro_rules! defer {
    ($e:expr) => {
        let _scope_call = ScopeCall {
            c: || -> () {
                $e;
            },
        };
    };
}
fn panic_if_true(param: bool) {
    //Guaranteed code execution
    //In case of any return or panic
    defer!(println!("defer"));
    println!("start");
    if param {
        println!("error");
        panic!("test panic");
    }
}

pub(crate) fn test() {
    panic_if_true(false);
    //printed:
    //start
    //defer
    panic_if_true(true);
    //printed:
    //start
    //error
    //defer
}
