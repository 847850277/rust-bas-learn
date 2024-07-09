//Employee is Product
trait Employee {
    fn test(&self);
}
//Booker is ConcreteProduct
struct Booker {}
impl Employee for Booker {
    fn test(&self) {
        println!("Booker");
    }
}
//Manager is ConcreteProduct
struct Manager {}
impl Employee for Manager {
    fn test(&self) {
        println!("Manager");
    }
}
//BookerCreator is ConcreteCreator
struct BookerCreator {}
impl BookerCreator {
    fn create_employee() -> Box<dyn Employee> {
        return Box::from(Booker{});
    }
}
//BookerCreator is ConcreteCreator
struct ManagerCreator {}
impl ManagerCreator {
    fn create_employee() -> Box<dyn Employee> {
        return Box::from(Manager{});
    }
}


pub(crate) fn test() {
    //Client
    let booker = BookerCreator::create_employee();
    booker.test();
    //printed: Booker
    let manager = ManagerCreator::create_employee();
    manager.test();
    //printed: Manager
}