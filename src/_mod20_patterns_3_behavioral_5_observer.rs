trait Observer {
    fn update(&self, state: &str);
}
//ConcreteObserver
struct TextObserver {
    name: String,
}
impl Observer for TextObserver {
    fn update(&self, state: &str) {
        println!("{}: {}", self.name, state);
    }
}
//ConcreteSubject
struct TextEdit {
    text: String,
    observers: Vec<Box<dyn Observer>>,
}
impl TextEdit {
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }
    #[allow(dead_code)]
    fn detach(&mut self, index: usize) {
        self.observers.remove(index);
    }
    fn notify(&self, state: &str) {
        for observer in &self.observers {
            observer.update(state);
        }
    }
    fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.notify(text);
    }
}

pub(crate) fn test() {
    //Client
    let observer1 = TextObserver {
        name: "Observer #1".to_string(),
    };
    let observer2 = TextObserver {
        name: "Observer #2".to_string(),
    };
    let mut text_edit = TextEdit {
        text: String::new(),
        observers: vec![],
    };
    text_edit.attach(Box::new(observer1));
    text_edit.attach(Box::new(observer2));
    text_edit.set_text("test text");
    //printed:
    //Observer #1: test text
    //Observer #2: test text
}
