use thiserror::Error;

#[derive(Error, Debug)]
enum AccessErr {
    #[error("Index is outside array bounds: {0}")]
    AccessOutsideBounds(usize)
}

fn get_at_idx(arr: &[u64], cap: usize, idx: usize) -> Result<u64, AccessErr> {
    if idx >= cap {
        return Err(AccessErr::AccessOutsideBounds(idx));
    }

    return Ok(arr[idx]);
}

pub fn bonus_start() {
    let arr = [0, 1, 2, 3, 4, 5, 6];
    match get_at_idx(&arr, std::mem::size_of_val(&arr) / std::mem::size_of_val(&arr[0]), 9) {
        Err(e_val) => println!("{}", e_val),
        Ok(i_val) => println!("{}", i_val)
    }
}