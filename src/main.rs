 use num_bigint::{BigInt, Sign};
// use num_traits::Pow;
use std::time::Instant;

fn main() {
    // let pi: f64 = leibniz(n);
    // println!("Calculation took {} minutes.", elapsed_time.as_secs()/60);
    // println!("Hello, Pi is {:.*}", 20, pi);
    for i in 0..=2 {
        let now = Instant::now();
        println!("chudnovsky({}): {:.*}", i, 20, chudnovsky(i));
        let elapsed_time = now.elapsed();
        println!("{} microsecs", elapsed_time.as_micros());
    }
}

/**
 * leibniz
 * 1/1 - 1/3 + 1/5 - 1/7 + 1/9 - ... = pi/4
 * infinite sum of (-1)^n+1(2n+1)^-1 where n starts at 0
 */

fn leibniz(n: i128) -> f64 {
    let mut neg: i128 = 1;
    let mut pi: f64 = 0.0;

    for i in 0..n {
        pi += (neg * 4) as f64 / (2*i+1) as f64;
        neg *= -1;
    }

    pi
}

/**
 * chudnovsky
 * (infinite sum where q starts at 0)
 * C = 426880*sqrt(10005)
 * multinomial = (6q)! / (3q)!(q!)^3
 * exponential = -262537412640768000^q
 * linear = 545140134q + 13591409
 * pi = C *  exponential / multinomial * linear
 */
fn chudnovsky(i: i128) -> f64 {

    /* overflow problem now */
    // can we utilize u64 up until the bitter end?
    // let base: BigInt = BigInt::new( Sign::Minus, Vec::from([2,6,2,5,3,7,4,1,2,6,4,0,7,6,8,0,0,0]) );
    let base: i128 = -262537412640768000;
    let C: f64 = 426880.0 * 10005.0_f64.sqrt();
    let mut multinomial: i128 = 0;
    let mut linear: i128 = 0;
    let mut exponential: i128 = 1; // BigInt = BigInt::new( Sign::Plus, Vec::new() );
    let mut pi: i128 = 0;

    // infinite sum
    for q in 0..i {
        exponential = base.pow(i as u32); // Pow::pow( base, i as u32);
        linear = 545140134 * n + 13591409;
        multinomial = factorial(6*n) /
        ( factorial(3*n) * factorial(n).pow(3) );
        pi += exponential /* as f64 */ / (multinomial * linear); // as f64;
    
        // println!("loop {}: pi = {:.*}", n, 20, pi);
        println!("pi {}: {}", n, pi);
    }
    C * pi as f64
}

/* linear = 545140134q + 13591409 */
// usize for now since its ran in a for loop
fn chud_linear(q: usize) -> BigInt { 
    result_at_q = BigInt::new( Sign::Plus, Vec::from[5,4,5,1,4,0,1,3,4]);
    reslut_at_q *= q;
    reslut_at_q += 13591409;
    result_at_q
}

/* exponential = -262537412640768000^q */
// usize for now since its ran in a forloop
fn chud_exponential(q: usize) -> BigInt { 
    // quick and dirty.. there should be a lib doing this for me,
    // but I will just impl my own until the compiler yells at me.
    let base: BigInt = BigInt::new( Sign::Minus, Vec::from([2,6,2,5,3,7,4,1,2,6,4,0,7,6,8,0,0,0]) );
    let mut result: BigInt = BigInt::new(Sign::Plus, 1); // base^0
    
    // exponentiation!
    for i in 0..q { result *= base; } // base * base q times
    result
}

/* multinomial = (6q)! / (3q)!(q!)^3 */
// usize for now since its ran in a forloop
fn chud_multinomial(q: usize) -> BigInt { 
        multinomial = factorial(6*n) /
        ( factorial(3*n) * factorial(n).pow(3) );
}

fn factorial(n: usize) -> BigInt {
    let mut fact_that_shit: BigInt = BigInt::new(Sign::Plus, 1);
    for i in 1..=n { fact_that_shit *= i; }
    fact_that_shit
}
