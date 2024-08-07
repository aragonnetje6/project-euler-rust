pub fn p085() -> u32 {
    let mut answers = get_all_areas(100);
    answers.sort_by_cached_key(|(_w, _h, n, _a)| n.abs_diff(2_000_000));
    // println!(
    //     "n: {}, a: {}",
    //     answers.first().unwrap().2,
    //     answers.first().unwrap().3
    // );
    answers.first().unwrap().3
}

fn get_rects_for(width: u32, height: u32) -> u32 {
    let mut total = 0;
    for i in 0..width {
        for j in 0..height {
            total += (width - i) * (height - j);
        }
    }
    total
}

fn get_all_areas(max_width: u32) -> Vec<(u32, u32, u32, u32)> {
    let mut out: Vec<(u32, u32, u32, u32)> = Vec::new();
    for width in 2..max_width {
        for height in 1..width {
            out.push((width, height, get_rects_for(width, height), width * height));
        }
    }
    out
}
