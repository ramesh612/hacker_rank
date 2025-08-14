use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

use std::f64::consts::SQRT_2;
//use core::f64::math::sqrt;

/*
 * Complete the 'movingTiles' function below.
 *
 * The function is expected to return a DOUBLE_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER l
 *  2. INTEGER s1
 *  3. INTEGER s2
 *  4. INTEGER_ARRAY queries
 */

fn moving_tiles(l: i32, s1: i32, s2: i32, queries: &[i64]) -> Vec<f64> {
    // println!("{}", SQRT_2);
    let mut replies: Vec<f64> = Vec::with_capacity(queries.len() as usize);
    for q in queries {
        replies
            .push((((SQRT_2 * (*q as f64).sqrt()) - (SQRT_2 * l as f64)) / (s1 - s2) as f64).abs());
    }
    replies
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let l = first_multiple_input[0].trim().parse::<i32>().unwrap();

    let s1 = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let s2 = first_multiple_input[2].trim().parse::<i32>().unwrap();

    let queries_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut queries: Vec<i64> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<i64>()
            .unwrap();
        queries.push(queries_item);
    }

    let result = moving_tiles(l, s1, s2, &queries);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
