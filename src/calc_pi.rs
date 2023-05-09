#![allow(non_snake_case)]
use num_bigint::{BigInt, ToBigInt};
use num_bigfloat::BigFloat;
use num_rational::BigRational;
use num_traits::{One, Zero};
use std::str::FromStr;

/**
 * leibniz
 * 1/1 - 1/3 + 1/5 - 1/7 + 1/9 - ... = pi/4
 * infinite sum of (-1)^n+1(2n+1)^-1 where n starts at 0
 */

pub fn leibniz(q: usize) -> BigFloat {
    let mut neg: BigInt = One::one();
    let mut qfinite_sum: BigRational = BigRational::zero();
    for k in 0..=q {
        qfinite_sum += BigRational::new( neg.clone(), (2 * k) + BigInt::one() );
        neg *= -1;
    }

    qfinite_sum *= BigRational::one() + BigRational::one() + BigRational::one() + BigRational::one();
    BigFloat::from_str( &qfinite_sum.numer().to_string() ).unwrap() / BigFloat::from_str( &qfinite_sum.denom().to_string() ).unwrap()
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
pub fn chudnovsky(q: usize) -> BigFloat {

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
        qfinite_sum += BigRational::new(linear.clone(), exponential.clone()) * multinomial.clone();
    }

    // take the inverse after the sumation and multiply by C
    BigFloat::from_str( &qfinite_sum.denom().to_string() ).unwrap() / BigFloat::from_str( &qfinite_sum.numer().to_string() ).unwrap() * C
}
