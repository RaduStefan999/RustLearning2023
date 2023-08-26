use std::{fs, io};
use serde::Deserialize;

#[derive(Deserialize)]
struct Student
{
    name: String,
    phone: String,
    age: u32
}

fn load_from_file(file_path: &str) -> Result<Vec<Student>, io::Error>
{
    let mut students: Vec<Student> = Vec::new();
    let text_file = fs::read_to_string(file_path)?;

    for line in text_file.lines()
    {
        if line.len() <= 2
        {
            continue;
        }

        let student_deserialized: Result<Student, _> = serde_json::from_str(&line);
        
        if student_deserialized.is_ok()
        {
            students.push(student_deserialized.unwrap());
        }
    }

    return Ok(students);
}

pub fn prob4_start()
{
    let students_result = load_from_file(r"D:\personal\RustLabs\RustLearning2023\lab3\res\prob4_file.txt");
    
    if let Err(err) = students_result
    {
        println!("{}", err);
        return;
    }

    let mut students = students_result.unwrap();
    students.sort_by(|lh, rh| lh.age.cmp(&rh.age));

    for student in students
    {
        println!("{}, {}, {}", student.name, student.phone, student.age);
    }
}