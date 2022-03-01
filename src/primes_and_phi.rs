use std::collections::{HashMap, HashSet};

pub fn get_all_phi_under(n: i32) -> HashMap<i32, i32> {
    let primes = get_all_primes_under(n);
    let primes_set: HashSet<i32> = HashSet::from_iter(primes.clone());
    let mut phis: HashMap<i32, i32> = HashMap::new();
    phis.insert(1, 1);
    for (i, decomposition) in to_powers(get_all_factors_under(n, &primes, &primes_set)) {
        phis.insert(
            i,
            decomposition.iter().fold(1, |acc, (prime, pow)| {
                acc * prime.pow((pow - 1) as u32) * (prime - 1)
            }),
        );
    }
    return phis;
}

pub fn to_powers(factors: HashMap<i32, Vec<i32>>) -> HashMap<i32, HashMap<i32, i32>> {
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

pub fn get_all_factors_under(
    n: i32,
    primes: &Vec<i32>,
    primes_set: &HashSet<i32>,
) -> HashMap<i32, Vec<i32>> {
    let mut factors: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 2..n {
        factors.insert(i, get_factors(i, &primes, &primes_set, &factors));
    }
    return factors;
}

fn get_factors(
    n: i32,
    primes: &Vec<i32>,
    primes_set: &HashSet<i32>,
    factors: &HashMap<i32, Vec<i32>>,
) -> Vec<i32> {
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

pub fn get_all_primes_under(max: i32) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    out.push(2);
    for i in 3..max {
        if is_prime(i, &out) {
            out.push(i);
        }
    }
    return out;
}

fn is_prime(n: i32, primes: &Vec<i32>) -> bool {
    for prime in primes {
        if prime.pow(2) > n {
            return true;
        }
        if &n % prime == 0 {
            return false;
        }
    }
    return true;
}
