struct Man<'a> {
    name: &'a str,
}
impl<'a> Man<'_> {
    fn new(name: &'a str) -> Man<'a> {
        return Man { name: name };
    }
}
struct Employee<'a> {
    parent: Man<'a>,
    position: &'a str,
}
impl<'a> Employee<'a> {
    fn new(name: &'a str, position: &'a str) -> Employee<'a> {
        return Employee {
            parent: Man::new(name),
            position: position,
        };
    }
    fn name(&self) -> &'a str {
        return self.parent.name;
    }
}

pub(crate) fn test() {
    let employee = Employee::new("Max", "booker");
    //employee.name() is "Max"
    println!("name is '{}'", employee.name());
    println!("position is '{}'", employee.position)
}
