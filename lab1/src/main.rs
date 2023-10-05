mod prob1;
mod prob2;
mod prob3;
mod prob4;
mod prob5;
mod bonus;


fn main() {
    prob1::prob1_start();
    prob2::prob2_start();
    match prob3::prob3_start() {
        Err(e) => println!("{}", e.get()),
        _ => ()
    }
    prob4::prob4_start();
    prob5::prb5_start();
    bonus::bonus_start();
}
