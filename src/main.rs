use std::time::{Instant, Duration};
use piece_of_pi::calc_pi;

fn main() {
    let i = 250;

    let now = Instant::now();
    println!("Leib({})::Pi = {}",i,  calc_pi::leibniz(i)); 
    let elapsed_time = now.elapsed();

    // TODO match statement to report based on how long it took.
    println!("{} milliseconds", elapsed_time.as_millis());

    
    let now = Instant::now();
    println!("Chud({})::Pi = {}",i,  calc_pi::chudnovsky(i)); 
    let elapsed_time = now.elapsed();

    // TODO match statement to report based on how long it took.
    println!("{} milliseconds", elapsed_time.as_millis());

    let now = Instant::now();
    println!("Chud({})::Pi = {}",i,  calc_pi::BIGDchudnovsky(i)); 
    let elapsed_time = now.elapsed();

    // TODO match statement to report based on how long it took.
    println!("{} milliseconds", elapsed_time.as_millis());


    println!("BigFloat::Pi = {}", num_bigfloat::PI);
}
