use std::collections::{HashMap, HashSet};

use crate::p69::{get_all_primes_under};

pub fn p70() {
    let phis = get_all_phi_under(10_000_000);
    let (permutation_n, permutation_phi) = phis
        .iter()
        .filter(|(k, _v)| {**k != 1})
        .filter(|(k, v)| { is_permutation(**k, **v) })
        .map(|(k, v)| { (*k, *v) })
        .min_by(|(n, phi), (n2, phi2)| { ((*n as f32) / (*phi as f32)).total_cmp(&((*n2 as f32) / (*phi2 as f32)))})
        .expect("No value found");
    println!("Minimum permutation totient: n={}, Phi(n)={}, ratio={}", permutation_n, permutation_phi, (permutation_n as f32) / (permutation_phi as f32));
}

fn is_permutation(x: i32, y: i32) -> bool {
    unsafe {
        let mut sorted_x = x.to_string().as_mut_vec().clone();
        sorted_x.sort();
        let mut sorted_y = y.to_string().as_mut_vec().clone();
        sorted_y.sort();
        return sorted_x == sorted_y;
    }
}

fn get_all_phi_under(n: i32) -> HashMap<i32, i32> {
    let primes = get_all_primes_under(n);
    let primes_set: HashSet<i32> = HashSet::from_iter(primes.clone());
    println!("{} primes generated", primes.len());
    let mut phis: HashMap<i32, i32> = HashMap::new();
    phis.insert(1, 1);
    for (i, decomposition) in to_powers(get_all_factors_under(n, &primes, &primes_set)) {
        phis.insert(i, decomposition.iter()
            .fold(1,
                  |acc, (prime, pow)|
                      { acc * prime.pow((pow - 1) as u32) * (prime - 1) }));
    }
    println!("Phi's generated");
    return phis;
}

fn to_powers(factors: HashMap<i32, Vec<i32>>) -> HashMap<i32, HashMap<i32, i32>> {
    let mut out: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for (key, val) in factors {
        out.insert(key, entry_to_powers(val));
    }
    return out;
}

fn entry_to_powers(val: Vec<i32>) -> HashMap<i32, i32> {
    let mut entry: HashMap<i32, i32> = HashMap::new();
    for prime in val {
        match entry.insert(prime, 1) {
            None => {}
            Some(x) => {
                let y = entry.get_mut(&prime).expect("just inserted");
                *y += x;
            }
        }
    }
    return entry;
}

fn get_all_factors_under(n: i32, primes: &Vec<i32>, primes_set: &HashSet<i32>) -> HashMap<i32, Vec<i32>> {
    let mut factors: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 2..n {
        factors.insert(i, get_factors(i, &primes, &primes_set, &factors));
        if i % (n / 1000) == 0 {
            println!("{}.{}%", i / (n / 100), i / (n / 1000) % 10)
        }
    }
    println!("Factors generated");
    return factors;
}

fn get_factors(n: i32, primes: &Vec<i32>, primes_set: &HashSet<i32>, factors: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    if primes_set.contains(&n) {
        return vec![n];
    }
    for prime in primes {
        if &n % prime == 0 {
            let mut new = factors.get(&(&n / prime)).expect("phi not found").clone();
            new.push(*prime);
            return new;
        }
    }
    panic!("never should have come here")
}
