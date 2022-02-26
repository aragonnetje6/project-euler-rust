#![feature(is_sorted)]
#![feature(int_roundings)]
#![feature(total_cmp)]

use std::time::Instant;
use crate::p69::{get_all_prime_decompositions_under, get_all_primes_under, p69};
use crate::p70::{p70};

mod p69;
mod p70;

fn main() {
    // p69();
    p70();
    // let decomps = get_all_prime_decompositions_under(1000000);
    // println!("{}", decomps.len());
    // for (i, decomp) in &decomps {
    //     assert_eq!(*i, decomp.iter().fold(1, |acc, x| {acc * x}))
    // }
    // let mut keys = decomps.keys().collect::<Vec<_>>();
    // keys.sort();
    // for i in keys {
    //     println!("{}", i)
    // }
    // const N: usize = 205;
    // const REPS: u128 = 100;
    // let mut times: [(i32, u128); N] = [(0, 0); N];
    // for i in 1..N as u32 + 1 {
    //     let now = Instant::now();
    //     for _i in 0..REPS {
    //         get_all_primes_under(2_i32.pow(i));
    //     }
    //     let time = now.elapsed().as_micros() / REPS;
    //     println!("All primes under {} took {}us", 2_i32.pow(i), time);
    //     times[(i - 1) as usize] = (2_i32.pow(i), time);
    // }
    // for prime in primes {
    //     println!("{}", prime);
    // }
    // println!("Time passed: {}ms", endtime.as_millis());
}
