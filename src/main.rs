// use num_bigfloat::BigFloat;
use std::time::Instant;

use piece_of_pi::calc_pi;

fn main() {
    let i = 50;
    let now = Instant::now();
    println!("Leib({})::Pi = {}",i,  calc_pi::leibniz(i)); 
    println!("Chud({})::Pi = {}",i,  calc_pi::chudnovsky(i)); 
    let elapsed_time = now.elapsed();
    // TODO match statement to report based on how long it took.
    println!("{} milliseconds", elapsed_time.as_millis());
    println!("BigFloat::Pi = {}", num_bigfloat::PI);

}
