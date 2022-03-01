use std::array;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

pub fn p096() {
    let sudokus = process_file(get_sudokus());
    let mut result = 0;
    let mut total_time = 0;
    for sudoku in sudokus {
        let solved: [[i32; 9]; 9];
        let before = Instant::now();
        match elimination_solve_sudoku(&sudoku) {
            Ok(x) => solved = x,
            Err(err) => {
                panic!("{}", err)
            }
        }
        let time = before.elapsed().as_millis();
        total_time += time;
        println!("Took {}ms", time);
        for line in solved {
            println!(
                "{}",
                line.iter()
                    .fold("".to_string(), |acc, x| { format!("{} {}", acc, x) })
            )
        }
        result += solved[0][0] * 100 + solved[0][1] * 10 + solved[0][2];
    }
    println!("Result: {}, took {}ms", result, total_time);
}

fn get_sudokus() -> String {
    let path = Path::new("src/p096_sudoku.txt");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }
    return s;
}

fn process_file(str: String) -> Vec<[[i32; 9]; 9]> {
    let mut out: Vec<[[i32; 9]; 9]> = Vec::new();
    for (i, line) in str.split("\n").enumerate() {
        if i % 10 == 0 {
            out.push([[0; 9]; 9]);
            continue;
        }
        let current_arr = out.last_mut().expect("just created");
        for (j, num) in line
            .chars()
            .map(|x| x.to_string().parse::<i32>().expect("all valid ints"))
            .enumerate()
        {
            current_arr[i % 10 - 1][j] = num;
        }
    }
    return out;
}

fn elimination_solve_sudoku(sudoku: &[[i32; 9]; 9]) -> Result<[[i32; 9]; 9], &str> {
    let mut options: [[HashSet<i32>; 9]; 9] =
        array::from_fn(|_| array::from_fn(|_| HashSet::new()));
    for (y, row) in sudoku.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == 0 {
                (1..10).for_each(|i| {
                    options[y][x].insert(i);
                });
            } else {
                options[y][x].insert(*entry);
            }
        }
    }
    for _i in 0..10 {
        for (y, row) in options.clone().iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                if entry.len() == 1 {
                    remove_from_options(&mut options, x, y, &entry);
                }
            }
        }
    }
    let mut options_vec: [[Vec<i32>; 9]; 9] = array::from_fn(|_| array::from_fn(|_| Vec::new()));
    for (y, row) in options.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            options_vec[y][x].append(&mut Vec::from_iter(entry.iter().map(|x| *x)));
        }
    }
    let mut sudoku: [[i32; 9]; 9] = array::from_fn(|y| {
        array::from_fn(|x| {
            if options_vec[y][x].len() == 1 {
                *options_vec[y][x].first().expect("exists")
            } else {
                0
            }
        })
    });
    let out = smart_backtrack_solve_sudoku(options_vec, sudoku);
    return out;
}

fn smart_backtrack_solve_sudoku(
    options_vec: [[Vec<i32>; 9]; 9],
    sudoku: [[i32; 9]; 9],
) -> Result<[[i32; 9]; 9], &'static str> {
    for (y, row) in sudoku.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry != 0 {
                continue;
            }
            let mut newsudoku = sudoku.clone();
            for i in options_vec[y][x].iter() {
                newsudoku[y][x] = i.clone();
                if valid(&newsudoku, x, y) {
                    match smart_backtrack_solve_sudoku(options_vec.clone(), newsudoku) {
                        Err(_) => {}
                        Ok(x) => {
                            return Result::Ok(x);
                        }
                    }
                }
            }
            return Result::Err("No valid entry");
        }
    }
    return Result::Ok(sudoku);
}

fn remove_from_options(
    options: &mut [[HashSet<i32>; 9]; 9],
    x: usize,
    y: usize,
    entry: &&HashSet<i32>,
) {
    for sub in 0..9 {
        if sub != x {
            options[y][sub] = HashSet::from_iter(options[y][sub].difference(entry).map(|x| *x));
        }
        if sub != y {
            options[sub][x] = HashSet::from_iter(options[sub][x].difference(entry).map(|x| *x));
        }
    }
    let x_offset = x / 3 * 3;
    let y_offset = y / 3 * 3;
    for x_sub in x_offset..x_offset + 3 {
        for y_sub in y_offset..y_offset + 3 {
            if (x_sub != x) | (y_sub != y) {
                options[y_sub][x_sub] =
                    HashSet::from_iter(options[y_sub][x_sub].difference(entry).map(|x| *x));
            }
        }
    }
}

fn backtrack_solve_sudoku(sudoku: &[[i32; 9]; 9]) -> bool {
    for (y, row) in sudoku.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry != 0 {
                continue;
            }
            let mut newsudoku = sudoku.clone();
            for i in 1..10 {
                newsudoku[y][x] = i;
                if valid(&newsudoku, x, y) & backtrack_solve_sudoku(&newsudoku) {
                    return true;
                }
            }
            return false;
        }
    }
    return true;
}

fn valid(sudoku: &[[i32; 9]; 9], x: usize, y: usize) -> bool {
    let mut count_arr = [0; 10];
    sudoku[y]
        .iter()
        .for_each(|item| count_arr[*item as usize] += 1);
    count_arr[0] = 0;
    if count_arr.iter().any(|entry| *entry > 1) {
        return false;
    }
    count_arr = [0; 10];
    sudoku
        .iter()
        .for_each(|item| count_arr[item[x] as usize] += 1);
    count_arr[0] = 0;
    if count_arr.iter().any(|entry| *entry > 1) {
        return false;
    }
    count_arr = [0; 10];
    let x_offset = x / 3 * 3;
    let y_offset = y / 3 * 3;
    for x in x_offset..x_offset + 3 {
        for y in y_offset..y_offset + 3 {
            count_arr[sudoku[y][x] as usize] += 1;
        }
    }
    count_arr[0] = 0;
    return !count_arr.iter().any(|entry| *entry > 1);
}
