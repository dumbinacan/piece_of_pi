#![allow(unused)]
use num_bigint::{BigInt, Sign, ToBigInt};
use num_bigfloat::BigFloat;
use num_rational::BigRational;
use num_traits::{cast::ToPrimitive, One, Zero};
use std::{str::FromStr, time::Instant};

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
    // for i in 0..18
    // { println!("Chudn({})::Pi = {}",i,  chudnovsky(i)); }
    let i = 50;
    // let now = Instant::now();
    println!("Chud({})::Pi = {}",i,  chudnovsky(i)); 
    // let elapsed_time = now.elapsed();
    // println!("{} microsecs", elapsed_time.as_millis());
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

    let C = BigFloat::from_f64( 426880.0 * 10005.0_f64.sqrt() );
    

    let mut K: BigInt = BigInt::zero() - 6; // K_0
    let mut exponential: BigInt = BigInt::one(); // X_0
    let mut linear = ToBigInt::to_bigint(&13591409).unwrap(); // L_0
    let mut multinomial: BigRational = BigRational::one(); // M_0
    let mut qfinite_sum: BigRational = BigRational::new( linear.clone(), One::one() ); // q = 0

    // qfinite sum
    for i in 1..=q {
        K += 12;
        multinomial *= BigRational::new( K.clone().pow(3) - 16 * K.clone(), (BigInt::one() + i).pow(3) ); // M_i
        linear += ToBigInt::to_bigint(&545140134).unwrap(); // L_i
        exponential *= ToBigInt::to_bigint( &(262537412640768000 as u128) ).unwrap() * -1; // X_i
        qfinite_sum += BigRational::new(linear.clone(), exponential.clone()) * multinomial.clone(); // .numer().clone(),
                                        // exponential.clone() * multinomial.denom().clone());
    }

    // take the inverse after the sumation and multiply by C
    BigFloat::from_str( &qfinite_sum.denom().to_string() ).unwrap() / BigFloat::from_str( &qfinite_sum.numer().to_string() ).unwrap() * C
}
