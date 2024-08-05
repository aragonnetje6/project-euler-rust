pub fn p068() {
    let result_3 = find_max_n_gon_string::<6>();
    println!("max string for 3-gon: {result_3:?}");
    let result_4 = find_max_n_gon_string::<8>();
    println!("max string for 4-gon: {result_4:?}");
    let result_5 = find_max_n_gon_string::<10>();
    println!("max string for 5-gon: {result_5:?}");
    let result_6 = find_max_n_gon_string::<12>();
    println!("max string for 6-gon: {result_6:?}");
    let result_7 = find_max_n_gon_string::<14>();
    println!("max string for 7-gon: {result_7:?}");
}

fn find_max_n_gon_string<const N: usize>() -> Option<u128> {
    let mut data = [0; N];
    for i in 0..N {
        data[i] = u32::try_from(i).unwrap() + 1;
    }
    Permutations::new(&mut data)
        .filter(|x| verify(x))
        .map(|x| stringify(&x))
        .filter(|x| x.len() < 17)
        .map(|x| x.parse().unwrap())
        .max()
}

fn stringify(layout: &[u32]) -> String {
    let rows: Vec<[u32; 3]> = layout
        .iter()
        .chain(layout.iter().take(2))
        .map_windows(|[a, b, _, c]| [**a, **b, **c])
        .enumerate()
        .filter_map(|(i, x)| (i % 2 == 0).then_some(x))
        .collect();
    let lowest_index = rows.iter().enumerate().min_by_key(|(_, x)| x[0]).unwrap().0;
    rows.iter()
        .skip(lowest_index)
        .chain(rows.iter().take(lowest_index))
        .map(|x| x.iter().map(u32::to_string).collect::<String>())
        .collect()
}

#[derive(Debug)]
struct Permutations<'a, const N: usize, T: Clone> {
    data: &'a mut [T; N],
    c: Vec<usize>,
    i: usize,
}

impl<'a, const N: usize, T: Clone> Permutations<'a, N, T> {
    fn new(data: &'a mut [T; N]) -> Self {
        Self {
            data,
            c: vec![0; N],
            i: 0,
        }
    }
}

impl<'a, const N: usize, T: Clone> Iterator for Permutations<'a, N, T> {
    type Item = [T; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            self.i = 1;
            return Some(self.data.clone());
        }
        if self.i == self.data.len() {
            return None;
        }
        if self.c[self.i] < self.i {
            if self.i % 2 == 0 {
                self.data.swap(0, self.i);
            } else {
                self.data.swap(self.c[self.i], self.i);
            }
            self.c[self.i] += 1;
            self.i = 1;
            return Some(self.data.clone());
        }
        self.c[self.i] = 0;
        self.i += 1;
        self.next()
    }
}

fn verify(layout: &[u32]) -> bool {
    layout
        .iter()
        .chain(layout.iter().take(2))
        .map_windows(|[a, b, _, c]| **a + **b + **c)
        .enumerate()
        .filter_map(|(i, x)| (i % 2 == 0).then_some(x))
        .map_windows(|[a, b]| a == b)
        .all(|x| x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify() {
        let mut data = [4, 3, 6, 2, 5, 1];
        assert!(verify(&data));
        data[0] = 5;
        assert!(!verify(&data));
    }

    #[test]
    fn test_permute() {
        let mut data = [4, 3, 6, 2, 5, 1];
        let cloned = data;
        let mut reversed = data;
        reversed.reverse();
        assert!(Permutations::new(&mut data).any(|x| x == cloned));
        assert!(Permutations::new(&mut data).any(|x| x == reversed));
        assert_eq!(Permutations::new(&mut data).count(), 720);
    }
}
