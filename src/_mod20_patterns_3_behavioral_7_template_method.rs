trait Shape {
    //Template method
    fn draw(&self) {
        if !self.can_draw() {
            return;
        }
        self.do_draw();
        self.notify_listeners();
    }
    fn can_draw(&self) -> bool {
        //If it possible to draw the shape
        return true;
    }
    //primitive operation
    fn do_draw(&self);
    fn notify_listeners(&self) {
        println!("shape is drawn");
    }
}
struct Circle {}
impl Shape for Circle {
    fn do_draw(&self) {
        println!("draw a circle");
    }
}

pub(crate) fn test() {
    //Client
    let circle = Circle {};
    circle.draw();
}
