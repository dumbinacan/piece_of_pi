/**
 * leibniz
 * 1/1 - 1/3 + 1/5 - 1/7 + 1/9 - ... = pi/4
 * infinite sum of (-1)^n+1(2n+1)^-1 where n starts at 0
 */

fn leibniz(n: i64) -> f64 {
    let neg = 1;
    let pi: f64 = 0.0;

    for i in 0..n {
        pi += (neg * 1) / (2*i+1)
        neg *= -1;
    }

    pi * 4
}
