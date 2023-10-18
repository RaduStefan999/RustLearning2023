fn add_chars_n(mut s: String, c: char, nr: u32) -> String {
    for _ in 0..nr {
        s.push(c);
    }
    return s;
}

fn add_chars_n_ref(s: &mut String, c: char, nr: u32) {
    for _ in 0..nr {
        s.push(c);
    }
}

fn add_space(s: &mut String, nr: u32) {
    for _ in 0..nr {
        s.push(' ');
    }
}

fn add_str(s: &mut String, sir: &str) {
    s.push_str(sir);
}

fn add_integer(s: &mut String, nr: u64) {
    s.push_str(nr.to_string().as_str());
}

fn add_float(s: &mut String, nr: f64) {
    s.push_str(nr.to_string().as_str());
}

fn prob3() {
    let mut sir = String::from("");
    let sir_ref = &mut sir;

    add_space(sir_ref, 40);
    add_str(sir_ref, "I");
    add_space(sir_ref, 1);
    add_str(sir_ref, "💚");
    add_str(sir_ref, "\n");

    add_space(sir_ref, 40);
    add_str(sir_ref, "RUST.");
    add_str(sir_ref, "\n");

    add_str(sir_ref, "\n");

    add_space(sir_ref, 4);
    add_str(sir_ref, "Most");
    add_space(sir_ref, 12);
    add_str(sir_ref, "crate");
    add_space(sir_ref, 6);
    add_integer(sir_ref, 306);
    add_str(sir_ref, "_");
    add_integer(sir_ref, 437);
    add_str(sir_ref, "_");
    add_integer(sir_ref, 968);
    add_space(sir_ref, 11);
    add_str(sir_ref, "and");
    add_space(sir_ref, 5);
    add_str(sir_ref, "lastest");
    add_space(sir_ref, 9);
    add_str(sir_ref, "is");
    add_str(sir_ref, "\n");

    add_space(sir_ref, 9);
    add_str(sir_ref, "downloaded");
    add_space(sir_ref, 8);
    add_str(sir_ref, "has");
    add_space(sir_ref, 13);
    add_str(sir_ref, "downloads");
    add_space(sir_ref, 5);
    add_str(sir_ref, "the");
    add_space(sir_ref, 9);
    add_str(sir_ref, "version");
    add_space(sir_ref, 4);
    add_float(sir_ref, 2.038);
    add_str(sir_ref, ".");

    print!("{}", sir);

}


fn main() {
    prob3();
    
    println!("\n\n\n");

    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        s = add_chars_n(s, c, 26 - i);
        add_chars_n_ref(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}