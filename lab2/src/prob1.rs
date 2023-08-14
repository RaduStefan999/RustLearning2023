use std::{io, fs};

enum LongestLinesError
{
    IO(io::Error),
    Empty()
}

impl LongestLinesError
{
    fn print(&self)
    {
        match &self {
            LongestLinesError::IO(err) => println!("IO error"),
            LongestLinesError::Empty() => println!("Empty text file")
        }
    }
}

fn read_and_get_longest_lines(file_path: &str) -> Result<(String, String), LongestLinesError>
{
    let text;

    match fs::read_to_string(file_path)
    {
        Err(err) => return Err(LongestLinesError::IO(err)),
        Ok(str) => text = str
    }

    if text.len() == 0
    {
        return Err(LongestLinesError::Empty());
    }

    let mut big_byte_line = "";
    let mut big_char_line = "";

    for line in text.lines()
    {
        if line.len() > big_byte_line.len()
        {
            big_byte_line = line;
        }
        if line.chars().count() > big_char_line.chars().count()
        {
            big_char_line = line;
        }
    }

    return Ok((String::from(big_char_line), String::from(big_byte_line)));
}

pub fn prob1_start()
{
    match read_and_get_longest_lines(r"D:\personal\RustLabs\RustLearning2023\lab2\res\prob1_file.txt") 
    {
        Err(err) => err.print(),
        Ok(res) => println!("Line with longest nr of chars: {}\nLine with longest nr of bytes: {}", res.0, res.1)    
    }
}