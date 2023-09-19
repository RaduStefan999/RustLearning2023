fn byte_to_hex_char(val: u8) -> char
{
    assert!(val < 16);
    if val < 10 {
        return (b'0' + val) as char;
    }
    return (b'A' + (val - 10)) as char;
}

fn hex_char_to_byte(val: char) -> u8
{
    let val_ascii = val.to_ascii_uppercase() as u8;
    if val_ascii >= b'A' && val_ascii <= b'F' {
        return val_ascii - b'A';
    }
    else if val_ascii >= b'0' && val_ascii <= b'9' {
        return val_ascii - b'0';
    }
    assert!(false);
    return 0;
}

pub fn byte_to_hex(val: u8) -> (char, char)
{
    let big_slice = (val & 0b1111_0000) >> 4;
    let small_slice = val & 0b0000_1111;
    return (byte_to_hex_char(big_slice), byte_to_hex_char(small_slice));
}

pub fn hex_to_byte(val: (char, char)) -> u8
{
    let big_slice = hex_char_to_byte(val.0);
    let small_slice = hex_char_to_byte(val.1);
    return (big_slice << 4) | small_slice;
}