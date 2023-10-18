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

fn i_to_str(mut nr: u64) -> String {
    let mut rev = String::from("");

    while nr > 0 {
        let ch = char::from(b'0' + ((nr%10) as u8));
        rev.push(ch);
        nr /= 10;
    }

    return rev.chars().rev().collect();
}

fn add_integer(s: &mut String, nr: u64) {
    let nr = i_to_str(nr);

    for (idx, ch) in nr.chars().enumerate() {
        if idx % 3 == 0 && idx != 0 {
            s.push('_');
        }
        s.push(ch);
    }
}

fn add_float(s: &mut String, nr: f32) {
    let i_part = nr as u64;
    let mut f_part = nr - (i_part as f32) + 1.0;
    while f_part - ((f_part as u64) as f32) > 0.01  {
        f_part *= 10.0;
    }
    let f_part = f_part as u64;

    s.push_str(&i_to_str(i_part));
    s.push('.');
    s.push_str(&i_to_str(f_part)[1..]);
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
    add_integer(sir_ref, 306_437_968);
    add_space(sir_ref, 11);
    add_str(sir_ref, "and");
    add_space(sir_ref, 5);
    add_str(sir_ref, "lastest");
    add_space(sir_ref, 9);
    add_str(sir_ref, "is");
    add_str(sir_ref, "\n");

    add_space(sir_ref, 9);
    add_str(sir_ref, "downloaded");
    add_space(sir_ref, 7);
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

fn prob3_smart() {
    let mut spaces: String = String::from("");
    add_space(&mut spaces, 2);

    let mut version: String = String::from("");
    add_float(&mut version, 2.038);
    add_str(&mut version, ".");

    let mut nr: String = String::from("");
    add_integer(&mut nr, 306_437_968);

    let words = [
        spaces,
        String::from("Most"), 
        String::from("downloaded"), 
        String::from("crate"),
        String::from("has"), 
        nr,
        String::from("downloads"), 
        String::from("and"),
        String::from("the"),
        String::from("lastest"),
        String::from("version"),
        String::from("is"),
        version
    ];

    let mut build_sir = String::from("");
    let sir_ref = &mut build_sir;

    add_space(sir_ref, 40);
    add_str(sir_ref, "I");
    add_space(sir_ref, 1);
    add_str(sir_ref, "💚");
    add_str(sir_ref, "\n");

    add_space(sir_ref, 40);
    add_str(sir_ref, "RUST.");
    add_str(sir_ref, "\n");

    add_str(sir_ref, "\n");

    for it in (1..words.len()).step_by(2) {
        add_space(&mut build_sir, words[it - 1].len() as u32 + 2);
        add_str(&mut build_sir, &words[it]);
    }
    add_space(&mut build_sir, words[words.len() - 1].len() as u32 + 2);
    add_str(&mut build_sir, "\n");

    add_space(&mut build_sir, 1);
    for it in (0..words.len()).step_by(2) {
        add_str(&mut build_sir, &words[it]);
        if it < words.len() - 1 {
            add_space(&mut build_sir, words[it + 1].len() as u32 + 2);
        }
    }
    print!("{}", build_sir);

}

fn main() {
    prob3();
    println!("\n\n\n");
    prob3_smart();
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