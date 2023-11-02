use std::{io, fs};

#[derive(Debug)]
enum ReadAbrevationsError
{
    IO(io::Error),
    Format()
}

fn read_abrevations(file_path: &str) -> Result<Vec<(String, String)>, ReadAbrevationsError>
{
    let mut result: Vec<(String, String)> = Vec::new();
    let fd = fs::read_to_string(file_path);

    if fd.is_err()
    {
        return Err(ReadAbrevationsError::IO(fd.unwrap_err()));
    }

    for line in fd.unwrap().lines()
    {
        if line.len() < 3
        {
            continue;
        }
        let mut split_line = line.split(" ");
        let word = split_line.nth(0);
        let abrevation = split_line.nth(0);

        if word.is_none() || abrevation.is_none()
        {
            return Err(ReadAbrevationsError::Format());
        }

        result.push((String::from(word.unwrap()), String::from(abrevation.unwrap())));
    }

    return Ok(result);
}


fn replace_abrevations(input_text: &str, abrevation_definitions: Vec<(String, String)>) -> String
{
    let mut resulting_words = String::with_capacity(input_text.len() * 5);
    let operation_text = String::from(input_text);

    'outer: for word in operation_text.split(" ")
    {
        for (abrevated_word, abrevation) in &abrevation_definitions
        {
            if word == abrevation
            {
                if !resulting_words.is_empty() {
                    resulting_words.push_str(" ");
                }
                
                resulting_words.push_str(abrevated_word);
                continue 'outer;
            }
        }
        
        if !resulting_words.is_empty() {
            resulting_words.push_str(" ");
        }

        resulting_words.push_str(word);
    }

    return resulting_words;
}

fn substitute_abrevations()
{
    let input_text = fs::read_to_string(r"D:\personal\RustLabs\RustLearning2023\lab2\res\prob3_input_file.txt");

    if input_text.is_err()
    {
        println!("{:?}", input_text.unwrap_err());
        return;
    }

    let input_abrevations = read_abrevations(r"D:\personal\RustLabs\RustLearning2023\lab2\res\prob3_abrevations_file.txt");

    if input_abrevations.is_err()
    {
        match input_abrevations.unwrap_err()
        {
            ReadAbrevationsError::IO(err) => println!("{:?}", err),
            ReadAbrevationsError::Format() => println!("Abrevation file is not formated correctly")
        }
        return;
    }
    
    let text_to_convert = input_text.unwrap();

    let replaced_text = replace_abrevations(text_to_convert.as_str(), input_abrevations.unwrap());

    println!("{}\n=>\n{}", text_to_convert, replaced_text);
}

pub fn prob3_start()
{
    substitute_abrevations();
}