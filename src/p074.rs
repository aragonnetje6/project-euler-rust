pub fn p074() {
    let mut total = 0;
    for i in 0..1000000 {
        let chain = get_chain(i);
        let non_rep_len = chain.len();
        if non_rep_len == 60 {
            total += 1;
        }
    }
    println!("Total 60-chains: {}", total);
}

fn factorial(x: u32) -> u32 {
    return if (x == 0) | (x == 1) {
        1
    } else {
        factorial(x - 1) * x
    };
}

fn digit_factorial(x: u32) -> u32 {
    x.to_string()
        .chars()
        .map(|x| factorial(x.to_string().parse::<u32>().expect("parse error")))
        .fold(0, |acc, i| acc + i)
}

fn get_chain(x: u32) -> Vec<u32> {
    let mut current = x.clone();
    let mut chain: Vec<u32> = Vec::new();
    while !chain.contains(&current) {
        chain.push(current);
        current = digit_factorial(current);
    }
    return chain;
}
