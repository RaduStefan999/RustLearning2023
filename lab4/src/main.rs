mod prob1;
mod prob2;
mod prob3;
mod prob4;
mod bonus;

use std::time::Instant;

fn main() {
    prob1::prob1_start();
    prob2::prob2_start();
    prob3::prob3_start();
    prob4::prob4_start();
    
    bonus::bonus_generate();

    let start = Instant::now();
    bonus::bonus_unoptimized();
    println!("{:?}", start.elapsed());
    // 20.8825281s - Unoptimized

    let start = Instant::now();
    bonus::bonus_optimize();
    println!("{:?}", start.elapsed());
    // 8.6987288s - Optimized
}
