//Handler
trait Rescuer {
    fn to_help(&self);
    //HandleRequest()
    fn help(&self, code: u8);
    fn try_help(&self, code: u8, rescuer: Option<&dyn Rescuer>) {
        if self.get_code() == code {
            self.to_help();
        } else {
            if let Some(next) = rescuer {
                next.help(code);
            }
        }
    }
    fn get_code(&self) -> u8;
}
//ConcreteHandler
struct Firefighter {
    next: Box<dyn Rescuer>,
    code: u8,
}
impl Rescuer for Firefighter {
    fn to_help(&self) {
        println!("call firefighters");
    }
    fn get_code(&self) -> u8 {
        return self.code;
    }
    fn help(&self, code: u8) {
        self.try_help(code, Some(self.next.as_ref()));
    }
}
//ConcreteHandler
struct Police {
    next: Box<dyn Rescuer>,
    code: u8,
}
impl Rescuer for Police {
    fn to_help(&self) {
        println!("call the police");
    }
    fn get_code(&self) -> u8 {
        return self.code;
    }
    fn help(&self, code: u8) {
        self.try_help(code, Some(self.next.as_ref()));
    }
}
//ConcreteHandler
struct Ambulance {
    code: u8,
}
impl Rescuer for Ambulance {
    fn to_help(&self) {
        println!("call an ambulance");
    }
    fn get_code(&self) -> u8 {
        return self.code;
    }
    fn help(&self, code: u8) {
        self.try_help(code, None);
    }
}

pub(crate) fn test() {
    let ambulance = Ambulance { code: 3 };
    let police = Police {
        next: Box::new(ambulance),
        code: 2,
    };
    let firefighter = Firefighter {
        next: Box::new(police),
        code: 1,
    };
    firefighter.help(1);
    //printed: call firefighters
    firefighter.help(2);
    //printed: call the police
    firefighter.help(3);
    //printed: call an ambulance
}
