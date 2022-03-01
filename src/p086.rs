use std::cmp::min;

pub fn p086() {
    println!("{}", get_shortest_path(6, 5, 3))
}

fn get_shortest_path(w: i32, d: i32, h: i32) -> f32 {
    return (min((w + d).pow(2) + h.pow(2), min((w + h).pow(2) + d.pow(2), (d + h).pow(2) + w.pow(2))) as f32).sqrt();
}