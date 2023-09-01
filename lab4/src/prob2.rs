use std::fs;
use std::fs::File;
use std::fmt::Write;
use std::collections::HashMap;
use std::io::Write;
use serde::Deserialize;

#[derive(Deserialize)]
struct EmojiData
{
    name: String,
    category: String,
    group: String,
    htmlCode: Vec<String>,
    unicode: Vec<String>
}

fn get_emoji_data_raw(remote_uri: &str) -> Result<String, std::io::Error>
{
    let resp = ureq::get(remote_uri).call().into_string()?;
    return Ok(resp);
}

fn get_emoji_data(remote_uri: &str) -> Option<Vec<EmojiData>>
{
    let data = get_emoji_data_raw(remote_uri);
    if let Err(err) = data
    {
        println!("Io error");
        return None;
    }
    let data = data.unwrap();

    let deserialized_data:  Result<Vec<EmojiData>, _> = serde_json::from_str(&data);
    if let Err(err) = deserialized_data
    {
        println!("Format error");
        return None;
    }

    return Some(deserialized_data.unwrap());
}

fn get_emoji_per_category(data: &Vec<EmojiData>) -> HashMap<&str, Vec<&EmojiData>>
{
    let mut result_map = HashMap::<&str, Vec<&EmojiData>>::new();

    for emoji in data
    {
        result_map.entry(emoji.category.as_str()).or_insert(Vec::<&EmojiData>::new()).push(emoji);
    }

    return result_map;
}

fn dump_categorized_data(categorized_data: &HashMap<&str, Vec<&EmojiData>>, output_path: &str) -> Result<(), std::io::Error>
{
    let mut result: String = String::new();

    for (key, value) in categorized_data
    {
        writeln!(result, "[{}]", key);

        for emoji in value
        {
            if emoji.unicode.is_empty() == false
            {
                writeln!(result, "[{}]", emoji.unicode[0]);
            }
        }
    }

    let mut file = File::create(output_path)?;
    file.write_all(result);

    return Ok(());
}

fn categorize(data: &Vec<EmojiData>)
{
    let categorized_data = get_emoji_per_category(data);
    dump_categorized_data(&categorized_data, r"D:\personal\RustLabs\RustLearning2023\lab4\res\prob2_output_file.txt");
}

pub fn prob2_start()
{
    if let Some(data) = get_emoji_data("http://127.0.0.1/get_data")
    {
        categorize(&data);
    }
}