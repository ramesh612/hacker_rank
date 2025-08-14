use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'find_point' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER px
 *  2. INTEGER py
 *  3. INTEGER qx
 *  4. INTEGER qy
 */

fn find_point(px: i32, py: i32, qx: i32, qy: i32) -> Vec<i32> {
    vec![qx + qx - px, qy + qy - py]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    for _ in 0..n {
        let first_multiple_input: Vec<String> = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|s| s.to_string())
            .collect();

        let px = first_multiple_input[0].trim().parse::<i32>().unwrap();

        let py = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let qx = first_multiple_input[2].trim().parse::<i32>().unwrap();

        let qy = first_multiple_input[3].trim().parse::<i32>().unwrap();

        let result = find_point(px, py, qx, qy);

        for i in 0..result.len() {
            write!(&mut fptr, "{}", result[i]).ok();

            if i != result.len() - 1 {
                write!(&mut fptr, " ").ok();
            }
        }

        writeln!(&mut fptr).ok();
    }
}
