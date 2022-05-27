#![feature(is_sorted)]
#![feature(int_roundings)]
#![allow(dead_code)]

extern crate core;

use crate::p054::p054;

// use crate::p067::p067;
// use crate::p069::p069;
// use crate::p070::p070;
// use crate::p071_072::p071;
// use crate::p074::p074;
// use crate::p075::p075;
// use crate::p085::p085;
// use crate::p086::p086;
// use crate::p087::p087;
// use crate::p092::p092;
// use crate::p094::p094;
// use crate::p096::{p096, solve_one};
// use crate::p097::p097;

mod file_io;
mod p054;
mod p067;
mod p069;
mod p070;
mod p071_072;
mod p074;
mod p075;
mod p085;
mod p086;
mod p087;
mod p092;
mod p094;
mod p096;
mod p097;
mod primes_and_phi;

fn main() {
    p054();
    // p067();
    // p069();
    // p070();
    // p071();
    // p074();
    // p075();
    // p085();
    // p086();
    // p087();
    // p092();
    // p094();
    // p096();
    // p097();

    // solve_one();
}
