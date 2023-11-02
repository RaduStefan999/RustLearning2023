use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufWriter;
use std::fs::File;
use std::time::Instant;

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
        if data[it] >= b'a' && data[it] <= b'z' {
            data[it] = rotate_lowercase(data[it], rot_by);
        }
        else if data[it] >= b'A' && data[it] <= b'Z' {
            data[it] = rotate_uppercase(data[it], rot_by);
        }
        
    }
}

const CHUNK_SIZE: usize = 64 * usize::pow(2, 13);

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

    loop
    {
        let n = match input.read(&mut buf_mem) {
            Ok(n) => n,
            Err(err) => { println!("Failed to read from file: {err}"); return; }
        };
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

fn main() {
    let start = Instant::now();
    rotate("input.txt", "output.txt", 13);
    println!("{:?}", start.elapsed());
}
