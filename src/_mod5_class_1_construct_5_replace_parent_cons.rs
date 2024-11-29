struct Man {
    name: String,
}
impl Man {
    fn new(name: String) -> Man {
        return Man { name: name };
    }
}
struct Employee {
    parent: Man,
    position: String,
}
impl Employee {
    fn new(name: String) -> Employee {
        return Employee {
            parent: Man::new(name),
            position: String::from("unknown"),
        };
    }
    fn name(&self) -> &String {
        return &self.parent.name;
    }
}

pub(crate) fn test() {
    let employee = Employee::new(String::from("Max"));
    //employee.name() is "Max"
    println!("name is '{}'", employee.name());
    println!("position is '{}'", employee.position);
}
