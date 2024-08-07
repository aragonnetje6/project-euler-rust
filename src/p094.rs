pub fn p094() -> i128 {
    get_sum_of_all_integer_triangle_perimeters_under(1_000_000_000)
}

fn has_integer_area(twosides: i128, oneside: i128) -> bool {
    let area_sqr = 4 * twosides.pow(2) - oneside.pow(2);
    for i in (area_sqr as f32).sqrt().floor() as i128..area_sqr {
        if i.pow(2) == area_sqr {
            return (i * oneside) % 4 == 0;
        } else if i.pow(2) > area_sqr {
            return false;
        }
    }
    false
}

fn get_integer_almost_equi_triangle_perimeters_from(length: i128) -> Vec<i128> {
    let mut out = Vec::with_capacity(3);
    if length > 1 && has_integer_area(length, length - 1) {
        out.push(get_perimeter(length, length - 1));
    }
    if has_integer_area(length, length) {
        out.push(get_perimeter(length, length));
    }
    if has_integer_area(length, length + 1) {
        out.push(get_perimeter(length, length + 1));
    }
    out
}

fn get_sum_of_all_integer_triangle_perimeters_under(n: i128) -> i128 {
    let mut total: i128 = 0;
    for length in 1..(n / 3 + 100) {
        let triangles = get_integer_almost_equi_triangle_perimeters_from(length);
        for perimeter in triangles {
            if perimeter <= n {
                total += perimeter;
                println!("perimeter: {perimeter}, total: {total}");
            }
        }
        if length % 10_000_000 == 0 {
            println!("currently at length {length}, total: {total}");
        }
    }
    total
}

fn get_perimeter(two_sides: i128, one_side: i128) -> i128 {
    2 * two_sides + one_side
}
