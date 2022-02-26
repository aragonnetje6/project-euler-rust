use std::collections::{HashMap};
use std::time::Instant;

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

fn get_prime_decomposition(n: i32, primes: &Vec<i32>, decompositions: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    if primes.contains(&n) {
        return vec![n];
    }
    for prime in primes {
        if &n % prime == 0 {
            let mut out: Vec<i32> = decompositions.get(&(&n / prime)).expect("decomposition not found").clone();
            out.push(*prime);
            return out;
        }
    }
    panic!("never should have come here")
}

pub fn get_all_prime_decompositions_under(n: i32) -> HashMap<i32, Vec<i32>> {
    let primes = get_all_primes_under(n);
    println!("{} primes generated", primes.len());
    let mut decompositions: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 2..n {
        decompositions.insert(i, get_prime_decomposition(i, &primes, &decompositions));
        if i % (n / 1000) == 0 {
            println!("{}.{}%", i / (n / 100), i / (n / 1000) % 10)
        }
    }
    println!("100.0%");
    return decompositions;
}

pub fn get_all_phi_under(n: i32) -> HashMap<i32, i32> {
    let mut phis: HashMap<i32, i32> = HashMap::new();
    let decompositions = get_all_prime_decompositions_under(n);
    println!("Decompositions done");
    for (i, decomposition) in decompositions {
        phis.insert(i, phi(to_hashmap(decomposition)));
    }
    println!("Phi's generated");
    return phis;
}

pub fn to_hashmap(arr: Vec<i32>) -> HashMap<i32, i32> {
    let mut out: HashMap<i32, i32> = HashMap::new();
    for item in arr {
        let count = out.entry(item).or_insert(0);
        *count += 1;
    }
    return out;
}

fn phi(decomposition: HashMap<i32, i32>) -> i32 {
    decomposition.iter().fold(1, |current, (prime, pow)| { current * prime.pow((pow - 1) as u32) * (prime - 1) })
}

fn get_phi_ratios_under(n: i32) -> HashMap<i32, f32> {
    return HashMap::from_iter(get_all_phi_under(n).iter().map(|(k, v)| { (k.clone(), *k as f32 / *v as f32) }).collect::<Vec<_>>());
}

fn get_max_phi_ratio_under(n: i32) -> (i32, f32) {
    let ratios = get_phi_ratios_under(n);
    let (max_n, max_ratio) = ratios.iter().max_by(|(_k1, v1), (_k2, v2)| { v1.partial_cmp(v2).expect("NAN or INF found") }).expect("maximum fail");
    return (max_n.clone(), max_ratio.clone());
}

pub fn p69() {
    let before = Instant::now();
    let (max_n, max_ratio) = get_max_phi_ratio_under(1000001);
    println!("Done! took {}s", before.elapsed().as_secs_f32());
    println!("N={}, ratio={}", max_n, max_ratio);
}