use std::fs;
use std::fs::File;
use std::fmt::Write as FormatWrite;
use std::io::Write as FileWrite;
use std::collections::HashMap;
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
        if let Err(err) = writeln!(result, "[{}]", key)
        {
            println!("Failed to write emoji category");
            break;
        }

        for emoji in value
        {
            if emoji.unicode.is_empty()
            {
                continue;
            }

            let emoji_code = u32::from_str_radix(&emoji.unicode[0][2..], 16);
            if let Err(err) = emoji_code
            {
                println!("Error getting emoji code");
                continue;
            }

            let emoji_char = char::from_u32(emoji_code.unwrap());
            if let None = emoji_char
            {
                println!("Error converting unicode to valid glyph");
                continue;
            }

            if let Err(err) = writeln!(result, "{} - {}", emoji_char.unwrap(), emoji.name)
            {
                println!("Failed to write emoji");
                continue;
            }
        }
    }

    let mut file = File::create(output_path)?;
    file.write_all(result.as_bytes())?;

    return Ok(());
}

fn categorize(data: &Vec<EmojiData>)
{
    let categorized_data = get_emoji_per_category(data);
    if let Err(err) = dump_categorized_data(&categorized_data, r"D:\personal\RustLabs\RustLearning2023\lab4\res\prob2_output_file.txt")
    {
        println!("{err}")
    }
}

pub fn prob2_start()
{
    if let Some(data) = get_emoji_data("http://127.0.0.1/get_data")
    {
        categorize(&data);
    }
}