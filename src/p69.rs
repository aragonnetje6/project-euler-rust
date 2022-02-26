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

// fn get_prime_decomposition_pollard(n: i32, primes: &Vec<i32>, decompositions: &HashMap<i32, HashMap<i32, i32>>) -> HashMap<i32, i32> {
//     let mut out: HashMap<i32, i32> = HashMap::new();
//     if primes.contains(&n) {
//         out.insert(n, 1);
//         return out;
//     }
//     let factor = pollard(n, primes);
//     if factor == -1 {
//         return get_prime_decomposition(n, primes, decompositions);
//     }
//     // println!("{}: {}", n, factor);
//     match decompositions.get(&factor) {
//         None => { panic!("no decomposition present") }
//         Some(partial) => { out = partial.clone() }
//     }
//     match decompositions.get(&(n / factor)) {
//         None => { panic!("no decomposition present") }
//         Some(partial) => {
//             for (prime, pow) in partial {
//                 match out.insert(*prime, *pow) {
//                     None => {}
//                     Some(x) => {
//                         let val = out.get_mut(&prime).expect("should also work");
//                         *val += x;
//                     }
//                 }
//             }
//         }
//     }
//     return out;
// }
//
// fn pollard(n: i32, primes: &Vec<i32>) -> i32 {
//     let mut b = 5;
//     let mut prev_b: HashSet<i32> = HashSet::from([b]);
//     loop {
//         let mut m: Vec<i32> = Vec::new();
//         for q in primes {
//             if q > &b {
//                 break;
//             } else {
//                 m.push(q.clone().pow((b as f32).log(*q as f32).floor() as u32));
//             }
//         }
//         let a: i32 = primes.iter().find(|x| { n % **x != 0 }).expect("PRIME LIST EMPTY").clone();
//         let a_to_m = m.iter().fold(a as i64, |acc, val| { mod_exp::mod_exp(acc, *val as i64, n as i64) }) as i32;
//         let g: i32 = gcd(a_to_m - 1, n);
//         if g == 1 {
//             b *= 2;
//         } else if g == n {
//             b -= 1;
//         } else {
//             return g;
//         }
//         if prev_b.contains(&b) {
//             return -1;
//         }
//         prev_b.insert(b);
//         // println!("N: {}, B: {}", n, b)
//     }
// }
//
// fn gcd(x: i32, y: i32) -> i32 {
//     let mut a = x.clone();
//     let mut b = y.clone();
//     while b != 0 {
//         let t = b;
//         b = a % b;
//         a = t;
//     }
//     return a;
// }

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