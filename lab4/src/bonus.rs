use std::{fs, io::{self, BufRead, Read, Write}};
use crate::prob2;
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;

fn generate_txt_from_base(base: &str, size: usize) -> String 
{
    let mut result = String::with_capacity(size);

    for _ in 0..(size / base.as_bytes().len()) {
        result.push_str(base);
    }
    result
}

fn generate_txt_file_from_base(base: &str, size: usize, path: &str) -> io::Result<()> {
    let res = generate_txt_from_base(base, size);
    return fs::write(path, res);
}

pub fn bonus_generate() {
    let res = generate_txt_file_from_base(
        "Malina, totul va fi bine", 
        4 * usize::pow(2, 30), 
        r"D:\personal\RustLabs\RustLearning2023\lab4\res\big_file.txt"
    );

    match res {
        Ok(_) => print!("Created file"),
        Err(err) => print!("{:?}", err)
    }
}

pub fn bonus_unoptimized() {

    prob2::read_and_rot_13(r"D:\personal\RustLabs\RustLearning2023\lab4\res\big_file.txt", 
    Some(r"D:\personal\RustLabs\RustLearning2023\lab4\res\encoded_big_file.txt"));    
}

fn rotate_in_place(data: &mut [u8], rot_by: u8) {
    let rotate_fn = |ch: u8, a_cod: u8, z_cod: u8, rot_by: u8| -> u8 {
        return (ch - a_cod + rot_by) % (z_cod - a_cod + 1) + a_cod;
    };
    let rotate_lowercase = |ch: u8, rot_by: u8| rotate_fn(ch, b'a', b'z', rot_by);
    let rotate_uppercase = |ch: u8, rot_by: u8| rotate_fn(ch, b'A', b'Z', rot_by);

    for it in 0..data.len() {
        if data[it] > 127 {
            panic!("This string should only contain ASCII");
        }
        if data[it] >= b'a' && b'z' <= data[it] {
            data[it] = rotate_lowercase(data[it], rot_by);
        }
        else if data[it] >= b'A' && b'Z' <= data[it] {
            data[it] = rotate_uppercase(data[it], rot_by);
        }
        
    }
}

const CHUNK_SIZE: usize = 64 * usize::pow(2, 10);

fn rotate(input_path: &str, output_path: &str, rot_by: u8) {

    let input_file = match File::open(input_path) {
        Ok(file_fd) => file_fd,
        Err(err) => { print!("Failed to open input file: {:?}", err); return; }
    };

    let output_file = match File::create(output_path) {
        Ok(file_fd) => file_fd,
        Err(err) => { print!("Failed to open output file: {:?}", err); return; }
    };

    let mut input = BufReader::new(input_file);
    let mut output = BufWriter::new(output_file);

    let mut buf_mem = [0u8; CHUNK_SIZE];

    while let Ok(n) = input.read(&mut buf_mem) {
        if n == 0 {
            break;
        }

        rotate_in_place(&mut buf_mem, rot_by);

        match output.write(&buf_mem) {
            Ok(_) => (),
            Err(err) => println!("Error writing to output file {err}")
        }
    }
}

pub fn bonus_optimize() {
    rotate(r"D:\personal\RustLabs\RustLearning2023\lab4\res\big_file.txt", r"D:\personal\RustLabs\RustLearning2023\lab4\res\encoded_big_file.txt", 13)
}