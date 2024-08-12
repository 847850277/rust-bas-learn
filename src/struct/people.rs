use std::sync::Arc;
use crate::r#struct::student::{Student, Student1};

#[derive(Debug)]
pub struct People{
    student : Student,
    string: String,
}


//new
impl People{
    pub fn new(student: Student, string: String) -> People{
        People{
            student,
            string
        }
    }

    pub fn new_empty() -> People{
        People{
            student: Student::new_empty(),
            string: "".to_string()
        }
    }
}

pub fn test1(){
    //new student ,运行将会报错
    let student = Student::new("John".to_string(), 20, 80, Arc::new(People::new_empty()));
    println!("{:?}", student);
    println!("test1");
}

#[derive(Debug,Clone)]
pub struct People1{
    student : Option<Student1>,
    string: String,
}


//new
impl People1{
    pub fn new(student: Option<Student1>, string: String) -> People1{
        People1{
            student,
            string
        }
    }

    pub(crate) fn set_student(&mut self, student: Student1) {
        self.student = Some(student);
    }

    pub fn new_empty() -> People1{
        People1{
            student: None,
            string: "".to_string()
        }
    }

}


pub(crate) fn test2() {
    let mut people =  People1::new_empty();
    let mut student1 = Student1::new("John".to_string(), 20, 80, Arc::new(people.clone()));
    dbg!(student1.clone());
    people.set_student(student1.clone());
    dbg!(people.clone());
    student1.set_people(Arc::new(people.clone()));
    dbg!(student1.clone());

}