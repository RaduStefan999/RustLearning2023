use std::{io, fs, ops::Deref};

#[derive(Clone)]
struct StudentData
{
    name: String,
    phone_number: String,
    age: u32
}

enum ReadIntoStructureErrors<'a>
{
    IO(io::Error),
    Format(&'a str)
}

fn read_into_structure(file_path: &str) -> Result<Vec<StudentData>, ReadIntoStructureErrors>
{
    let mut result: Vec<StudentData> = Vec::new();
    let text: String;

    match fs::read_to_string(file_path) {
        Err(err) => return Err(ReadIntoStructureErrors::IO(err)),
        Ok(ok_text) => text = ok_text
    }

    for line in text.lines()
    {
        let mut line_split_iter = line.split(',');

        let name = line_split_iter.nth(0);
        let phone_nr = line_split_iter.nth(0);
        let age = line_split_iter.nth(0);

        if name.is_none() || phone_nr.is_none() || age.is_none()
        {
            return Err(ReadIntoStructureErrors::Format("Cannot read field"));
        }

        let age_nr: Result<u32, _> =  age.unwrap().parse();

        if age_nr.is_err()
        {
            return Err(ReadIntoStructureErrors::Format("Age is not a number"));
        }

        let current_student = StudentData{
            name: String::from(name.unwrap()), 
            phone_number: String::from(phone_nr.unwrap()), 
            age: age_nr.unwrap()
        };

        result.push(current_student);
    }

    return Ok(result);
}

fn sort_and_print(data: &Vec<StudentData>)
{
    let mut data_clone: Vec<StudentData> = data.deref().to_vec();
    data_clone.sort_by(|lh, rh| lh.age.cmp(&rh.age));

    for student in data_clone
    {
        println!("Name: {}, Phone number: {}, Age: {}", student.name, student.phone_number, student.age);
    }
}

pub fn prob1_start()
{
    let result = read_into_structure(r"D:\personal\RustLabs\RustLearning2023\lab3\res\prob1_file.txt");

    if let Err(result_err) = result
    {
        match result_err {
            ReadIntoStructureErrors::IO(err) => println!("{}", err),
            ReadIntoStructureErrors::Format(err) => print!("{}", err)
        }
        return;
    }
    else if let Ok(result_ok) = result
    {
        sort_and_print(&result_ok);
    }
}