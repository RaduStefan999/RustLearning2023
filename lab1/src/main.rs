mod prob1;
mod prob2;
mod prob3;

fn main() {
    //prob1::prob1_start();
    //prob2::prob2_start();
    match prob3::prob3_start() {
        Err(e) => println!("{}", e.get()),
        _ => ()
    }
}
