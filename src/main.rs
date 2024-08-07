#![feature(int_roundings, iter_map_windows, iter_collect_into)]
#![allow(dead_code)]
#![warn(clippy::pedantic)]

extern crate core;

use crate::p054::p054;

use crate::p067::p067;
use crate::p068::p068;
use crate::p069::p069;
use crate::p070::p070;
use crate::p071_072::p071;
use crate::p074::p074;
use crate::p075::p075;
use crate::p085::p085;
use crate::p086::p086;
use crate::p087::p087;
use crate::p092::p092;
use crate::p094::p094;
use crate::p096::{p096, solve_one};
use crate::p097::p097;

mod file_io;
mod p054;
mod p067;
mod p068;
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
    println!("p054: {}", p054());
    println!("p054: {}", p067());
    println!("p054: {}", p068());
    println!("p054: {}", p069());
    println!("p054: {}", p070());
    println!("p054: {}", p071());
    println!("p054: {}", p074());
    println!("p054: {}", p075());
    println!("p054: {}", p085());
    println!("p054: {}", p086());
    println!("p054: {}", p087());
    println!("p054: {}", p092());
    println!("p054: {}", p094());
    println!("p054: {}", p096());
    println!("p054: {}", p097());

    solve_one();
}
