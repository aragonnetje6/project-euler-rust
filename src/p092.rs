pub fn p092() {
    let all_ends = get_all_digit_chain_ends_under(10000000);
    println!("{}", all_ends.iter().filter(|x| { **x == 89 }).count());
}

fn digit_square(x: u32) -> u32 {
    x.to_string()
        .chars()
        .map(|x| (x.to_string().parse::<u32>().expect("parse error")).pow(2))
        .fold(0, |acc, i| acc + i)
}

fn get_digit_chain_end(n: u32) -> u32 {
    let mut i = n;
    while (i != 89) & (i != 1) & (i != 0) {
        i = digit_square(i);
    }
    return i;
}

fn get_all_digit_chain_ends_under(n: u32) -> Vec<u32> {
    let mut ends = vec![0 as u32];
    for i in 1..n {
        ends.push(get_digit_chain_end_smart(i, &ends))
    }
    return ends;
}

fn get_digit_chain_end_smart(n: u32, ends: &Vec<u32>) -> u32 {
    let mut i = n;
    while (i != 89) & (i != 1) & (i != 0) {
        i = digit_square(i);
        match ends.get(i as usize) {
            None => {}
            Some(x) => { return *x; }
        }
    }
    return i;
}