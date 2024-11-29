use crate::r#struct::people::*;
use std::sync::Arc;

#[derive(Debug)]
pub struct Student {
    name: String,
    age: u8,
    marks: u8,
    people: Arc<People>,
}

//new
impl Student {
    pub fn new(name: String, age: u8, marks: u8, people: Arc<People>) -> Student {
        Student {
            name,
            age,
            marks,
            people,
        }
    }

    pub(crate) fn new_empty() -> Student {
        Student {
            name: "".to_string(),
            age: 0,
            marks: 0,
            people: Arc::new(People::new_empty()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Student1 {
    name: String,
    age: u8,
    marks: u8,
    people: Arc<People1>,
}

//new
impl Student1 {
    pub fn new(name: String, age: u8, marks: u8, people: Arc<People1>) -> Student1 {
        Student1 {
            name,
            age,
            marks,
            people,
        }
    }

    pub(crate) fn set_people(&mut self, people: Arc<People1>) {
        self.people = people;
    }
}
