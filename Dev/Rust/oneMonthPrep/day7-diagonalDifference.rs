use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();  // Square with n rows and n columns
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];  // Primary diagonal: 00, 11, 22, 33 etc...
        secondary_diagonal_sum += arr[i][n - 1 - i];  // Secondary diagonal 02, 11, 20 (in case of n=3)
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs()  //Absolute function, hell yeah rust.
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();  // Using usize for indexing

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        let row = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        arr.push(row);
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
