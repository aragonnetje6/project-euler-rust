pub fn p075() -> i32 {
    let mut total = 1;
    for i in 1..10000 {
        if has_one_triangle(i) {
            total += 1;
        }
    }
    total
}

fn has_one_triangle(l: u32) -> bool {
    let mut found_one = false;
    for h in 1..=(l / 3) {
        for w in h..l {
            let d = l - h - w;
            if (w > l / 2) | (h + w > 2 * l / 3) | (d < w) {
                break;
            }
            if d > l / 2 {
                continue;
            }
            if d.pow(2) == h.pow(2) + w.pow(2) {
                if found_one {
                    return false;
                }
                found_one = true;
            }
        }
    }
    found_one
}
