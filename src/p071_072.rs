use std::collections::{HashMap, HashSet};
use crate::primes_and_phi::{get_all_factors_under, get_all_phi_under, get_all_primes_under, to_powers};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Fraction {
    n: i32,
    d: i32,
}

impl Fraction {
    fn reduce(&self) -> Fraction {
        let div = gcd(self.n, self.d);
        return Fraction { n: self.n / div, d: self.d / div };
    }

    fn gcd(&self) -> i32 {
        return gcd(self.n, self.d);
    }

    fn is_reduced(&self) -> bool {
        return is_reduced(self);
    }

    fn eval(&self) -> f32 {
        return self.n as f32 / self.d as f32;
    }
}

fn gcd(x: i32, y: i32) -> i32 {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn is_reduced(frac: &Fraction) -> bool {
    return gcd(frac.n, frac.d) == 1;
}

pub fn p071() {
    const N: i32 = 1000000;
    const TARGET: Fraction = Fraction {n: 3, d: 7};
    // let primes = get_all_primes_under(N);
    // let primes_set = HashSet::from_iter(primes.clone());
    // let factors = to_powers(get_all_factors_under(N, &primes, &primes_set));
    // let count = get_all_phi_under(1000001).iter().fold(0, |acc, (_k, v)| {acc + *v as u128});
    // println!("Least common multiple={}", count);
    // let mut total_set = get_fractions_nominator_under(N);
    // println!("{} fractions generated", total_set.len());
    // total_set.sort_by(|f1, f2| {(f1.n as f32 / f1.d as f32).total_cmp(&(f2.n as f32 / f2.d as f32))});
    // println!("sorted");

    let fractions = get_close_fractions_under(TARGET, N);
    for frac in fractions.iter().max_by(|frac1, frac2| {frac1.eval().total_cmp(&frac2.eval())}) {
        println!("{}", format!("{:?}", frac));
    }
}

// fn get_lcm(factors: &HashMap<i32, HashMap<i32, i32>>) -> i128 {
//     let mut lcm_factors: HashMap<i32, i32> = HashMap::new();
//     for (_i, factor_map) in factors {
//         for (prime, pow) in factor_map {
//             let old = lcm_factors.insert(*prime, *pow);
//             match old {
//                 None => {}
//                 Some(x) => {if x > *pow {
//                     lcm_factors.insert(*prime, x);
//                 }}
//             }
//         }
//     }
//     return lcm_factors.iter().fold(1, |acc, (prime, pow)| {acc * (*prime as i128).pow(*pow as u32)});
// }

fn get_fractions_nominator_under(n: i32) -> Vec<Fraction> {
    let mut out: Vec<Fraction> = Vec::new();
    for i in 2..n {
        for j in 1..i {
            let frac = Fraction {n: j, d: i};
            if is_reduced(&frac) {
                out.push(frac);
            }
        }
    }
    return out;
}

fn get_close_fractions_under(frac: Fraction, d_max: i32) -> Vec<Fraction> {
    let mut out: Vec<Fraction> = Vec::new();
    let mut other_frac = frac.clone();
    while other_frac.d < d_max {
        other_frac.d += 1;
        other_frac.n += 1;
        if other_frac.eval() > frac.eval() {
            other_frac.n -= 1;
        }
        if other_frac.is_reduced() {
            out.push(other_frac.clone());
        }
    }
    return out;
}
