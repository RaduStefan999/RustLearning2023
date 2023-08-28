use std::{fs, io};
use std::fmt::Write;
use std::collections::HashMap;

#[derive(Debug)]
enum ComputeOutputErr
{
    IO(io::Error),
    Format()
}

fn convert_sentence_to_count_map(sentence: &str) -> HashMap<&str, u32>
{
    let mut result: HashMap<&str, u32> = HashMap::new();

    for word in sentence.split(" ")
    {
        if word.len() == 0
        {
            continue;
        }

        *(result.entry(word).or_insert(0)) += 1;
    }

    return result;
}

fn covert_vec_to_text(input_vec: &Vec<(&str, u32)>) -> Option<String>
{
    let mut result = String::new();
    let max_key_len = input_vec.into_iter().map(|it| it.0.len()).max()?;

    for (key, val) in input_vec
    {
        if writeln!(&mut result, "{}{}=> {}", key, " ".repeat(max_key_len - key.len()), val).is_err()
        {
            return None;
        }
    }

    return Some(result);
}

fn read_and_compute_output(file_path: &str) -> Result<String, ComputeOutputErr>
{
    let file_content;

    match fs::read_to_string(file_path) {
        Err(err) => return Err(ComputeOutputErr::IO(err)),
        Ok(res) => file_content = res.to_lowercase().replace(".", "")
    }

    let count_map = convert_sentence_to_count_map(&file_content);
    let mut count_map_entries = Vec::from_iter(count_map.into_iter());
    count_map_entries.sort_by(|lh, rh| lh.1.cmp(&rh.1).reverse());

    match covert_vec_to_text(&count_map_entries) {
        None => return Err(ComputeOutputErr::Format()),
        Some(res) => return Ok(res)
    }
}

pub fn prob1_start()
{
    match read_and_compute_output(r"D:\personal\RustLabs\RustLearning2023\lab4\res\prob1_file.txt") {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{:?}", err)
    }
}