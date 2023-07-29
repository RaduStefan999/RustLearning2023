fn checked_addition(lh: u32, rh: u32) -> u32 {
    if lh as u64 + rh as u64 > u32::MAX as u64 {
        panic!("Checked addition result does not fit in u32");
    }
    return lh + rh;
}

fn checked_multiplication(rh: u32, lh: u32) -> u32 {
    if lh as u64 * rh as u64 > u32::MAX as u64 {
        panic!("Checked multiplication result doaes not fit in u32");
    }

    return lh * rh;
}

pub fn prob2_start() {
    //println!("{}", checked_addition(10, 20));
    //println!("{}", checked_addition(u32::MAX, 1));
    //println!("{}", checked_multiplication(14, 24));
    println!("{}", checked_multiplication(1 << 16, 1 << 17));
}