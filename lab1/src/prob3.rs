pub enum OverflowError {
    Msg(String)
}

impl OverflowError {
    pub fn get(self) -> String {
        match self {
            OverflowError::Msg(s) => return s
        }
    }
}

fn checked_addition(lh: u32, rh: u32) -> Result<u32, OverflowError> {
    if lh as u64 + rh as u64 > u32::MAX as u64 {
        return Err(OverflowError::Msg(String::from("Checked addition result does not fit in u32")));
    }
    return Ok(lh + rh);
}

fn checked_multiplication(rh: u32, lh: u32) -> Result<u32, OverflowError> {
    if lh as u64 * rh as u64 > u32::MAX as u64 {
        return Err(OverflowError::Msg(String::from("Checked multiplication result doaes not fit in u32")));
    }
    return Ok(lh * rh);
}

pub fn prob3_start() -> Result<(), OverflowError> {
    println!("{}", checked_addition(10, 20)?);
    println!("{}", checked_addition(u32::MAX, 1)?);
    println!("{}", checked_multiplication(14, 24)?);
    println!("{}", checked_multiplication(1 << 16, 1 << 17)?);
    return Ok(());
}