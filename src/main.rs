use std::time::{Duration, Instant};
use piece_of_pi::{
    calc_pi,
    format_time::{TimeUnit, TimeFormatter},
};

fn main() {
    // let i = 250;
    let now = Instant::now();
    // println!("Leib({})::Pi = {}",i,  calc_pi::leibniz(i)); 
    // let elapsed_time = now.elapsed();
    // println!("It took {} to run", format_time(elapsed_time));

    // let now = Instant::now();
    // println!("Chud({})::Pi = {}",i,  calc_pi::chudnovsky(i)); 
    // let elapsed_time = now.elapsed();
    // println!("It took {} to run", format_time(elapsed_time));

    // let now = Instant::now();
    // println!("Chud({})::Pi = {}",i,  calc_pi::BIGDchudnovsky(i)); 
    // let elapsed_time = now.elapsed();
    // println!("It took {} to run", format_time(elapsed_time));

    // println!("BigFloat::Pi = {}", num_bigfloat::PI);


    let minute_nhalf = Duration::from_secs(90);
    let minute = Duration::from_secs(60);
    let second = Duration::from_secs(1);
    let nanosec = Duration::from_nanos(1);
    let microsec = Duration::from_micros(1);
    let millisec = Duration::from_millis(1);
    let format_time = TimeFormatter::new();

    let elapsed_time = millisec;
    println!("It took {} to run", format_time.format(elapsed_time));
/*
    let elapsed_time = microsec; 
    println!("It took {} to run", format_time.format(elapsed_time));

    let elapsed_time = nanosec;
    println!("It took {} to run", format_time.format(elapsed_time));

    let elapsed_time = second;
    println!("It took {} to run", format_time.format(elapsed_time));

    let elapsed_time = minute;
    println!("It took {} to run", format_time.format(elapsed_time));

    let elapsed_time = minute_nhalf;
    println!("It took {} to run", format_time.format(elapsed_time));

    let elapsed_time = now.elapsed();
    println!("It took {} to run", format_time.format(elapsed_time));
*/
} 
