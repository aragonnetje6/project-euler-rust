use std::cmp::min;

pub fn p086() -> i32 {
    
    // println!("m needed: {result}");
    // println!("gives {} paths", get_nr_of_integer_paths_under(result));
    get_least_m_with_more_than_n_integer_paths(1_000_000)
}

fn get_shortest_path(w: i32, d: i32, h: i32) -> f32 {
    (min(
        (w + d).pow(2) + h.pow(2),
        min((w + h).pow(2) + d.pow(2), (d + h).pow(2) + w.pow(2)),
    ) as f32)
        .sqrt()
}

fn is_integer_path(w: i32, d: i32, h: i32) -> bool {
    get_shortest_path(w, d, h) % 1. == 0.
}

fn update_nr_of_integer_paths(m: i32, count: i32) -> i32 {
    let w = m;
    let mut newcount = count;
    for d in 1..=m {
        for h in 1..=d {
            if is_integer_path(w, d, h) {
                newcount += 1;
            }
        }
    }
    newcount
}

fn get_nr_of_integer_paths_under(max_m: i32) -> i32 {
    let mut current_count = 0;
    for m in 1..=max_m {
        current_count = update_nr_of_integer_paths(m, current_count);
    }
    current_count
}

fn get_least_m_with_more_than_n_integer_paths(n: i32) -> i32 {
    let mut current_count = 0;
    let mut m = 0;
    while current_count < n {
        m += 1;
        current_count = update_nr_of_integer_paths(m, current_count);
    }
    m
}
