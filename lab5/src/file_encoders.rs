use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::BufReader;

#[path ="../src/hex_encoding.rs"]
mod hex_encoding;

const BUFFER_SIZE: usize = 4096;

fn read_file(path_to_file: &str, mut callback: impl FnMut(&[u8]) -> Result<(), std::io::Error>) -> Result<(), std::io::Error>
{
    let file = File::open(path_to_file);
    if let Err(err) = file
    {
        return Err(err);
    }
    let file = file.unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut buffer = [0u8; BUFFER_SIZE];

    while let Ok(quantity) = buf_reader.read(&mut buffer)
    {
        if quantity == 0
        {
            break;
        }
        callback(&buffer[0..quantity])?;
    }
    return Ok(());
}

pub fn encode_base_16(input_path: &str, output_path: &str) -> Result<(), std::io::Error>
{
    let mut fd = File::create(output_path)?;
    return read_file(input_path, |buf: &[u8]| -> Result<(), std::io::Error> {
        let mut result_holder = String::with_capacity(buf.len() * 2);
        for bt in buf
        {
            let hex_pair = hex_encoding::byte_to_hex(*bt);
            result_holder.push(hex_pair.0);
            result_holder.push(hex_pair.1);
        }
        write!(&mut fd, "{}", result_holder)?;
        return Ok(());
    });
}

pub fn decode_base_16(input_path: &str, output_path: &str) -> Result<(), std::io::Error>
{
    let mut fd = File::create(output_path)?;
    return read_file(input_path, |buf: &[u8]| -> Result<(), std::io::Error> {
        assert!(buf.len() % 2 == 0);
        let mut result_holder: Vec<u8> = Vec::with_capacity(buf.len() / 2);

        for it in (0..buf.len()).into_iter().step_by(2)
        {
            
            result_holder.push(hex_encoding::hex_to_byte((buf[it] as char, buf[it + 1] as char)));
        }

        fd.write(&result_holder)?;
        return Ok(());
    });
}
