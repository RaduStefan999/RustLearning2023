use std::{io, fs};

fn rotate_primitive(ch: char, a_cod: u32, z_cod: u32, rot_by: u32) -> char
{
    return char::from_u32((ch as u32 - a_cod + rot_by) % (z_cod - a_cod + 1) + a_cod).unwrap();
}

fn rotate_lowercase(ch: char, rot_by: u8) -> char
{
    return rotate_primitive(ch, 'a' as u32, 'z' as u32, rot_by as u32);
}

fn rotate_uppercase(ch: char, rot_by: u8) -> char
{
    return rotate_primitive(ch, 'A' as u32, 'Z' as u32, rot_by as u32);
}

fn rotate_ch(ch: char, rot_by: u8) -> Result<char, String>
{
    if !ch.is_ascii()
    {
        return Err(String::from("Given char is not ascii"));
    }

    if !ch.is_alphabetic()
    {
        return Ok(ch);
    }

    if ch.is_ascii_uppercase()
    {
        return Ok(rotate_uppercase(ch, rot_by));
    }

    return Ok(rotate_lowercase(ch, rot_by));
}

fn rotate_str_13(sir: &String) -> Result<String, String>
{
    let mut res: String = String::from("");

    for ch in sir.chars()
    {
        match rotate_ch(ch, 13) {
            Ok(rotated_ch) => res.push(rotated_ch),
            Err(error) => return Err(error)
        }
    }

    return Ok(res);
}

pub fn read_and_rot_13(path: &str, output_path: Option<&str>)
{
    let sir: String;
    
    if let Ok(res) = fs::read_to_string(path)
    {
        sir = res;
    }
    else 
    {
        println!("Error while reading from file {path}");
        return;
    }

    let rot_return = rotate_str_13(&String::from(sir));
    let contents = match rot_return {
        Ok(contents_res) => contents_res,
        Err(err) => { print!("{:?}", err); return; }
    };

    if let Some(path) = output_path {
        match fs::write(path, contents) {
            Ok(_) => print!("Written to file"),
            Err(err) => print!("{}", err)
        }
        return;
    }

    print!("{}", contents);
}

pub fn prob2_start()
{
    read_and_rot_13(r"D:\personal\RustLabs\RustLearning2023\lab2\res\prob2_file.txt", None);
}