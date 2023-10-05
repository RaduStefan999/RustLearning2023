fn is_nr_prime(nr: u16) -> bool {
    if nr < 2 {
        return false;
    }
    if nr == 2 {
        return true;
    }
    if nr % 2 == 0 {
        return false;
    }
    let mut it: u16 = 3;
    while it < ((nr as f32).sqrt() as u16 + 1) {
        if nr % it == 0 {
            return false;
        }
        it += 2;
    }

    return true;
}

fn find_next_prime(current_prime: u16) -> Option<u16> {
    let mut current_nr = current_prime;

    while current_nr + 1 < u16::MAX {
        current_nr += 1;

        if is_nr_prime(current_nr) {
            return Some(current_nr);
        }
    }

    return None;
}

pub fn prob1_start() {
    let mut current_number: u16 = 2;

    while let Some(u_val) = find_next_prime(current_number) {
        current_number = u_val;
        println!("{u_val}");
    }
}