enum CharOpErr {
    CharNotASCII,
    CharNotDigit,
    CharNotBase16,
    CharNotLetter,
    CharNotPrintable,
    UnexpectedErr
}

fn to_uppercase(val: char) -> Result<char, CharOpErr> {
    if !val.is_alphabetic() {
        return Err(CharOpErr::CharNotLetter);
    }

    for character in val.to_uppercase() {
        return Ok(character);
    }

    return Err(CharOpErr::UnexpectedErr);
}

fn to_lowercase(val: char) -> Result<char, CharOpErr> {
    if !val.is_alphabetic() {
        return Err(CharOpErr::CharNotLetter);
    }

    for character in val.to_lowercase() {
        return Ok(character);
    }

    return Err(CharOpErr::UnexpectedErr);
}

fn print_char(val: char) -> Result<(), CharOpErr> {
    if val >= '\x20' && val <= '\x7e' {
        println!("{val}");
        return Ok(());
    }
    return Err(CharOpErr::CharNotPrintable);
}

fn char_to_number(val: char) -> Result<u32, CharOpErr> {
    if !val.is_ascii() {
        return Err(CharOpErr::CharNotASCII);
    }

    if !val.is_ascii_digit() {
        return Err(CharOpErr::CharNotDigit);
    }

    match val.to_digit(10) {
        Some(i_val) => return Ok(i_val),
        _ => ()
    }

    return Err(CharOpErr::UnexpectedErr);
}

fn char_to_number_hex(val: char) -> Result<u32, CharOpErr> {
    if !val.is_ascii() {
        return Err(CharOpErr::CharNotASCII);
    }

    if !val.is_ascii_hexdigit() {
        return Err(CharOpErr::CharNotBase16);
    }

    match val.to_digit(16) {
        Some(i_val) => return Ok(i_val),
        _ => ()
    }

    return Err(CharOpErr::UnexpectedErr);
}

fn print_error(error: CharOpErr) {
    match error {
        CharOpErr::CharNotASCII => println!("Char is not ascii"),
        CharOpErr::CharNotDigit => println!("Char is not digit"),
        CharOpErr::CharNotBase16 => println!("Char is not base 16"),
        CharOpErr::CharNotLetter => println!("Char is not a letter"),
        CharOpErr::CharNotPrintable => println!("Char is not printable"),
        CharOpErr::UnexpectedErr => println!("An unexpected error that should never accour has happened")
    }
}

impl CharOpErr {
    fn print(self) {
        print_error(self);
    }
}

pub fn prob4_start() {
    match to_uppercase('a') {
        Err(err_code) => err_code.print(),
        Ok(char_val) => println!("To uppercase = {}", char_val)
    }

    match to_lowercase('A') {
        Err(err_code) => err_code.print(),
        Ok(char_val) => println!("To lowercase = {}", char_val)
    }

    match print_char('m') {
        Err(err_code) => err_code.print(),
        Ok(()) => ()
    }

    match char_to_number('8') {
        Err(err_code) => err_code.print(),
        Ok(num_val) => println!("Char to number = {}", num_val)
    }

    match char_to_number_hex('E') {
        Err(err_code) => err_code.print(),
        Ok(num_val) => println!("Hex char to number {}", num_val)
    }

    match char_to_number('a') {
        Err(err_code) => err_code.print(),
        Ok(num_val) => println!("Char to number = {}", num_val)
    }
}
