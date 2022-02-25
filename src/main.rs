#![feature(is_sorted)]
#![feature(int_roundings)]

use crate::p69::{p69};
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
}
