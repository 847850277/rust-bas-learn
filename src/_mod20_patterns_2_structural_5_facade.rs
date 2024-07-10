struct Kettle {}
impl Kettle {
    fn turn_off(&self) {
        println!("Kettle turn off");
    }
}
struct Toaster {}
impl Toaster {
    fn turn_off(&self) {
        println!("Toaster turn off");
    }
}
struct Refrigerator {}
impl Refrigerator {
    fn turn_off(&self) {
        println!("Refrigerator turn off");
    }
}
//Facade
struct Kitchen {
    kettle: Kettle,
    toaster: Toaster,
    refrigerator: Refrigerator,
}
impl Kitchen {
    fn off(&self) {
        self.kettle.turn_off();
        self.toaster.turn_off();
        self.refrigerator.turn_off();
    }
}

pub(crate) fn test() {

    let kettle = Kettle{};
    let toaster = Toaster{};
    let refrigerator = Refrigerator{};
    let kitchen = Kitchen{
        kettle: kettle,
        toaster: toaster,
        refrigerator: refrigerator };
    kitchen.off();

}