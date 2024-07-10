trait Element {
    fn accept(&self, v: &Box<dyn CarVisitor>);
}

//ConcreteElement
struct Engine {}
impl Element for Engine {
    fn accept(&self, v: &Box<dyn CarVisitor>) {
        v.visit_engine(self);
    }
}

//ConcreteElement
struct Wheel {
    number: i8
}
impl Element for Wheel {
    fn accept(&self, v: &Box<dyn CarVisitor>) {
        v.visit_wheel(self);
    }
}

//ConcreteElement
struct Car {
    items: Vec<Box<dyn Element>>
}
impl Car {
    fn new() -> Car {
        return Car {
            items: vec![ Box::new(Engine{}),
                         Box::new(Wheel{ number: 1 }),
                         Box::new(Wheel{ number: 2 }),
                         Box::new(Wheel{ number: 3 }),
                         Box::new(Wheel{ number: 4 })]
        };
    }
}
impl Element for Car {
    fn accept(&self, v: &Box<dyn CarVisitor>) {
        v.visit_car(self);
        for e in &self.items {
            e.accept(v);
        }
    }
}

//Visitor
trait CarVisitor {
    fn visit_engine(&self, engine: &Engine);
    fn visit_wheel(&self, wheel: &Wheel);
    fn visit_car(&self, car: &Car);
}

//ConcreteVisitor
struct TestCarVisitor {}
impl CarVisitor for TestCarVisitor {
    fn visit_engine(&self, _engine: &Engine) {
        println!("- test engine");
    }

    fn visit_wheel(&self, wheel: &Wheel) {
        println!("- test wheel #{}", wheel.number);
    }

    fn visit_car(&self, _car: &Car) {
        println!("test car:");
    }
}

//ConcreteVisitor
struct RepairCarVisitor {}
impl CarVisitor for RepairCarVisitor {
    fn visit_engine(&self, _engine: &Engine) {
        println!("- repair engine");
    }

    fn visit_wheel(&self, wheel: &Wheel) {
        println!("- repair wheel #{}", wheel.number);
    }

    fn visit_car(&self, _car: &Car) {
        println!("repair car:");
    }
}

pub(crate) fn test() {

//Client
    let car = Car::new();
    let v1: Box<dyn CarVisitor> = Box::new(
        TestCarVisitor{});
    let v2: Box<dyn CarVisitor> = Box::new(
        RepairCarVisitor{});
    car.accept(&v1);
    car.accept(&v2);

}