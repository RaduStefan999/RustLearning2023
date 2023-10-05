#[derive(Debug)]
pub enum OverflowError {
    AdditionFailed((u32, u32)),
    MultiplicationFailed((u32, u32))
}

impl OverflowError {
    pub fn get(self) -> String {
        match self {
            OverflowError::AdditionFailed(nums) => return format!("Checked addition between ({}, {}) result does not fit in u32", nums.0, nums.1),
            OverflowError::MultiplicationFailed(nums) => return format!("Checked multiplication between ({}, {}) result doaes not fit in u32", nums.0, nums.1)
        }
    }
}

fn checked_addition(lh: u32, rh: u32) -> Result<u32, OverflowError> {
    if lh as u64 + rh as u64 > u32::MAX as u64 {
        return Err(OverflowError::AdditionFailed((lh, rh)));
    }
    return Ok(lh + rh);
}

fn checked_multiplication(rh: u32, lh: u32) -> Result<u32, OverflowError> {
    if lh as u64 * rh as u64 > u32::MAX as u64 {
        return Err(OverflowError::MultiplicationFailed((lh, rh)));
    }
    return Ok(lh * rh);
}

pub fn prob3_start() -> Result<(), OverflowError> {
    println!("{:?}", checked_addition(10, 20)?);
    println!("{:?}", checked_addition(u32::MAX, 1)?);
    println!("{:?}", checked_multiplication(14, 24)?);
    println!("{:?}", checked_multiplication(1 << 16, 1 << 17)?);
    Ok(())
}