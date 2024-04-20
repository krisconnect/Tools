// Initial idea: iterate through the two arrays, sort one in ascending order, the other in descending order, compare the sum of each corresponding element to k and return YES if all the sums are at greater than or equal to k
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'twoArrays' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER k
 *  2. INTEGER_ARRAY A
 *  3. INTEGER_ARRAY B
 */

fn twoArrays(k: i32, A: &[i32], B: &[i32]) -> String {
    
    let mut A_copy = A.to_vec();
    let mut B_copy = B.to_vec();

    
    A_copy.sort_unstable();
    
    B_copy.sort_unstable_by(|a, b| b.cmp(a));

    
    for (a, b) in A_copy.iter().zip(B_copy.iter()) {
        if a + b < k {
            return "NO".to_string();
        }
    }

    "YES".to_string()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let _n = first_multiple_input[0].trim().parse::<i32>().unwrap();  // n is not actually needed for logic
        let k = first_multiple_input[1].trim().parse::<i32>().unwrap();

        let A: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let B: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let result = twoArrays(k, &A, &B);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
