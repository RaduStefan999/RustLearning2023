fn is_prime(val: u64) -> bool {
    if val < 2 {
        return false;
    }
    if val == 2 {
        return true;
    }
    if val % 2 == 0  {
        return false;
    }
    for it in (3..((val as f64).sqrt() as u64) + 1).step_by(2) {
        if val % it == 0 {
            return false;
        }
    }
    return true;
}

fn find_primes_in_0_100() {
    for it in 0..100 {
        if is_prime(it) {
            println!("{}", it);
        }
    }
}

fn are_coprime(mut lh: u64, mut rh: u64) -> bool {
    let cmmdc = {
        while rh != 0 {
            let aux = lh % rh;
            lh = rh;
            rh = aux;
        }
        lh
    };

    return cmmdc == 1;
}

fn find_coprimes_in_0_100() {
    for it in 0..100 {
        for jt in it..100 {
            let res = are_coprime(it, jt);
            if res {
                println!("({}; {})", it, jt);
            }
        }
    }
}

fn play_99_bottles() {
    let mut nr_of_bottles = 99;

    while nr_of_bottles != 0 {
        println!("{} bottles of beer on the wall,", nr_of_bottles);
        println!("{} bottles of beer.", nr_of_bottles);
        println!("Take one down, pass it around,");

        nr_of_bottles -= 1;
    }

    println!("No bottles of beer on the wall.");
}

fn main() {
    find_primes_in_0_100();
    find_coprimes_in_0_100();
    play_99_bottles();
}