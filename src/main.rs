#[derive(Debug, PartialEq)] // Để có thể sử dụng assert_eq! với Student
pub struct Student {
    pub name: String,
    pub age: u32,
    pub score: u32,
}

impl Student {
    pub fn new(name: &str, age: u32, score: u32) -> Student {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new("John", 18, 80);
        assert_eq!(student.name, "John");
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 80);
    }

    #[test]
    fn test_student_equality() {
        let student1 = Student::new("Alice", 17, 90);
        let student2 = Student::new("Alice", 17, 90);
        assert_eq!(student1, student2);
    }
}

use std::collections::HashMap;

pub struct Students {
    pub class: HashMap<String, Student>,
}

impl Students {
    pub fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    pub fn add(&mut self, student: Student) {
        self.class.insert(student.name.clone(), student);
    }

    pub fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }

    pub fn edit(&mut self, name: &str, age: u32, score: u32) -> bool {
        match self.class.get_mut(name) {
            Some(student) => {
                student.age = age;
                student.score = score;
                true
            }
            None => false,
        }
    }
}

mod manager {
    use super::*;

    pub fn add_student(students: &mut Students) {
        println!("Enter name of Student:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        println!("Enter Age of Student:");
        let age = match get_int() {
            Some(number) => number,
            None => return,
        };

        println!("Enter Score of Student:");
        let score = match get_int() {
            Some(number) => number,
            None => return,
        };

        let student = Student::new(&name, age, score);
        students.add(student);
    }

    pub fn view(students: &Students) {
        for student in students.view_all() {
            println!("{:#?}", student);
        }
    }

    pub fn remove_student(students: &mut Students) {
        println!("Enter name of Student to remove:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        if students.remove(&name) {
            println!("Student removed successfully.");
        } else {
            println!("Student not found.");
        }
    }

    pub fn edit_student(students: &mut Students) {
        println!("Enter name of Student to edit:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };

        println!("Enter new Age of Student:");
        let age = match get_int() {
            Some(input) => input,
            None => return,
        };

        println!("Enter new Score of Student:");
        let score = match get_int() {
            Some(input) => input,
            None => return,
        };

        if students.edit(&name, age, score) {
            println!("Student information updated successfully.");
        } else {
            println!("Student not found.");
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            let input = buffer.trim().to_owned();
            if input.is_empty() {
                None
            } else {
                Some(input)
            }
        }
        Err(_) => {
            println!("Error reading input.");
            None
        }
    }
}

fn get_int() -> Option<u32> {
    match get_input() {
        Some(input) => match input.parse::<u32>() {
            Ok(number) => Some(number),
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                None
            }
        },
        None => None,
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("\n== Manager Panel ==");
        println!("1. Add Student");
        println!("2. View Students");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("5. Exit");
        println!("Please Enter Your Choice:");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            "5" => None,
            _ => {
                println!("Invalid choice. Please enter a number from 1 to 5.");
                None
            }
        }
    }
}

fn run_program() {
    let mut students = Students::new();
    loop {
        Manager::show();
        let input = match get_input() {
            Some(input) => input,
            None => {
                println!("Error reading input.");
                continue;
            }
        };
        match Manager::choice(&input) {
            Some(Manager::AddStudent) => manager::add_student(&mut students),
            Some(Manager::ViewStudent) => manager::view(&students),
            Some(Manager::EditStudent) => manager::edit_student(&mut students),
            Some(Manager::DeleteStudent) => manager::remove_student(&mut students),
            None => break,
        }
    }
}

fn main() {
    run_program();
    println!("Exiting program.");
}
