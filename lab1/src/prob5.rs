enum CMMDCErrCodes {
    CouldNotParseStrings,
    OneNrIsZero
}

fn str_to_nr(sir: &String) -> Option<u64> {
    let mut num: u64 = 0;
    let sir_bytes = sir.as_bytes();
    
    for it in 0..sir.len() {
        if sir_bytes[it].is_ascii_digit() == false {
            return None;
        }

        num = num*10 + (sir_bytes[it] as u64 - 48);
    }

    return Some(num);
}

fn cmmdc(lh: &String, rh: &String) -> Result<u64, CMMDCErrCodes> {
    let mut lh_nr: u64;
    let mut rh_nr: u64;

    match str_to_nr(lh) {
        Some(i_val) => lh_nr = i_val,
        None => return Err(CMMDCErrCodes::CouldNotParseStrings)
    }
    match str_to_nr(rh) {
        Some(i_val) => rh_nr = i_val,
        None => return Err(CMMDCErrCodes::CouldNotParseStrings)
    }

    if rh_nr == 0 || lh_nr == 0 {
        return Err(CMMDCErrCodes::OneNrIsZero);
    }

    while rh_nr != 0 {
        let aux = lh_nr % rh_nr;
        lh_nr = rh_nr;
        rh_nr = aux;
    }

    return Ok(lh_nr);
}


pub fn prb5_start() {
    match cmmdc(&String::from("144"), &String::from("84")) {
        Ok(i_val) => println!("{}", i_val),
        Err(CMMDCErrCodes::CouldNotParseStrings) => println!("Invalid strings"),
        Err(CMMDCErrCodes::OneNrIsZero) => println!("One of the given numbers is zero")
        
    }
}