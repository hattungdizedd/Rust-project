// Project quản lý sinh viên bằng ngôn ngữ Rust


#[derive(Debug, PartialEq)] // Để có thể sử dụng assert_eq! với Student
pub struct Student {
    pub name: String,
    pub age: u32,
    pub score: u32,
}

impl Student {
    // Phương thức khởi tạo mới cho Student
    pub fn new(name: &str, age: u32, score: u32) -> Student {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }
}

// Định nghĩa các unit tests cho cấu trúc Student
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Test tạo mới sinh viên
    fn test_student_creation() {
        let student = Student::new("John", 18, 80);
        assert_eq!(student.name, "John");
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 80);
    }

    #[test]
    // Test so sánh hai sinh viên có giống nhau không
    fn test_student_equality() {
        let student1 = Student::new("Alice", 17, 90);
        let student2 = Student::new("Alice", 17, 90);
        assert_eq!(student1, student2);
    }
}

use std::collections::HashMap;

// Cấu trúc lưu trữ danh sách sinh viên
pub struct Students {
    pub class: HashMap<String, Student>,
}

impl Students {
    // Phương thức khởi tạo mới cho Students
    pub fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    // Phương thức thêm sinh viên mới vào danh sách
    pub fn add(&mut self, student: Student) {
        self.class.insert(student.name.clone(), student);
    }

    // Phương thức xem tất cả sinh viên trong danh sách
    pub fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }

    // Phương thức xóa sinh viên khỏi danh sách
    pub fn remove(&mut self, name: &str) -> bool {
        self.class.remove(name).is_some()
    }

    // Phương thức chỉnh sửa thông tin của sinh viên trong danh sách
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

// Module quản lý các hành động liên quan đến sinh viên
mod manager {
    use super::*;

    // Hàm thêm sinh viên mới vào danh sách
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

    // Hàm xem tất cả sinh viên trong danh sách
    pub fn view(students: &Students) {
        for student in students.view_all() {
            println!("{:#?}", student);
        }
    }

    // Hàm xóa sinh viên khỏi danh sách
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

    // Hàm chỉnh sửa thông tin của sinh viên trong danh sách
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

// Hàm utility để đọc đầu vào từ người dùng
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

// Hàm utility để đọc một số nguyên dương từ người dùng
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

// Enum Manager và các phương thức của nó
enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    // Phương thức hiển thị menu lựa chọn
    fn show() {
        println!("\n== Manager Panel ==");
        println!("1. Add Student");
        println!("2. View Students");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("5. Exit");
        println!("Please Enter Your Choice:");
    }

    // Phương thức xử lý lựa chọn của người dùng
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

// Hàm chính chạy ứ

ng dụng quản lý sinh viên
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

// Hàm main của chương trình
fn main() {
    run_program(); // Chạy chương trình quản lý sinh viên
    println!("Exiting program.");
}


