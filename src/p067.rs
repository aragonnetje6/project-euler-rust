use std::cmp::max;

use crate::file_io::read_file;

pub fn p067() -> i32 {
    let mut triangle = process_triangle(read_file("src/p067_triangle.txt"));
    triangle.reverse();
    calculate_max_path(&triangle)
}

fn process_triangle(str: String) -> Vec<Vec<i32>> {
    let mut out: Vec<Vec<i32>> = Vec::new();
    for line in str.split('\n').filter(|x| !x.is_empty()) {
        out.push(
            line.split(' ')
                .map(|x| x.parse().expect("fucking parse failure"))
                .collect(),
        );
    }
    out
}

fn calculate_max_path(triangle: &Vec<Vec<i32>>) -> i32 {
    let (first_row, all_but_first_triangle) = triangle.split_first().expect("empty triangle");
    let mut path_values: Vec<i32> = first_row.clone();
    for next_line in all_but_first_triangle {
        let mut next_level = Vec::new();
        for (i, value) in path_values.iter().enumerate() {
            if i < next_line.len() {
                next_level.push(
                    max(*value, *path_values.get(i + 1).expect("index error"))
                        + next_line.get(i).expect("index error 2"),
                );
            }
        }
        path_values = next_level;
    }
    return *path_values.first().expect("empty vec error");
}
