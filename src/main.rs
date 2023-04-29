use num_bigint::{BigInt, Sign, ToBigInt};
use num_bigfloat::BigFloat;
use num_traits::cast::ToPrimitive;
// use std::time::Instant;

fn main() {
    // let pi: f64 = leibniz(n);
    // println!("Calculation took {} minutes.", elapsed_time.as_secs()/60);
    // println!("Hello, Pi is {:.*}", 20, pi);
    // for i in 0..=2 {
        // let now = Instant::now();
        // println!("chudnovsky({}): {:.*}", i, 20, chudnovsky(i));
        // let elapsed_time = now.elapsed();
        // println!("{} microsecs", elapsed_time.as_micros());
    // }
    /*
    let mut big_int: BigInt = ToBigInt::to_bigint(&1).unwrap();
    let q = 2;
    big_int = chud_exponential(q);
    println!("chud_exponential({}) = {}", q, big_int);
    big_int = chud_linear(q);
    println!("chud_linear({}) = {}", q, big_int);
    big_int = chud_multinomial(q);
    println!("chud_multinomial({}) = {}", q, big_int);
    */
    println!("ThisCalc::Pi = {}", chudnovsky(3));
    println!("BigFloat::Pi = {}", num_bigfloat::PI);

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
fn chudnovsky(q: usize) -> BigFloat {

    /* overflow problem now */
    // can we utilize u64 up until the bitter end?
    let mut pi: BigFloat = BigFloat::new();
    let C = BigFloat::from_f64( 426880.0 * 10005.0_f64.sqrt() );

    // qfinite sum
    for i in 0..q {

        let numerator = chud_exponential(i);
        let denominator = chud_multinomial(i) * chud_linear(i);
        
        // HOWTO do this operation and get out an f64?
        // pi += (numerator / denominator).to_f64().unwrap(); // will this just work?
        // pi += (numerator % denominator).to_f64().unwrap() / denominator.to_f64().unwrap(); // the rational part
        
        pi += BigFloat::from_f64( numerator.to_f64().unwrap() / denominator.to_f64().unwrap() );
    
        // println!("loop {}: pi = {:.*}", i, 20, pi);
        // println!("pi {}: {}", i, pi);
    }
    C * pi
    // 0.0 as f64
}

/* linear = 545140134q + 13591409 */
// usize for now since its ran in a for loop
fn chud_linear(q: usize) -> BigInt { 
    let mut result_at_q = BigInt::new( Sign::Plus, Vec::from([5,4,5,1,4,0,1,3,4]) );
    result_at_q *= q;
    result_at_q += 13591409;
    result_at_q
}

/* exponential = -262537412640768000^q */
// usize for now since its ran in a forloop
fn chud_exponential(q: usize) -> BigInt { 
    BigInt::new( Sign::Minus, Vec::from([2,6,2,5,3,7,4,1,2,6,4,0,7,6,8,0,0,0]) ).pow(q as u32)
}

/* multinomial = (6q)! / (3q)!(q!)^3 */
// usize for now since its ran in a forloop
fn chud_multinomial(q: usize) -> BigInt { 
        /* (6q)! */         /* (3q)! */         /* (q!)^3 */
    factorial(6 * q) / ( factorial(3 * q) * (factorial(q)).pow(3) )
}

fn factorial(n: usize) -> BigInt {
    let mut fact_that_shit: BigInt = ToBigInt::to_bigint(&1).unwrap();
    for i in 1..=n { fact_that_shit *= i; }
    fact_that_shit
}
