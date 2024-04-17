use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) { 
    let total = arr.len() as f64; // Cast length of the array to f64 for floating point division
    let mut positive_count = 0.0;
    let mut negative_count = 0.0;
    let mut zero_count = 0.0;

    // Count positive, negative, and zero values
    for &value in arr {
        if value > 0 {
            positive_count += 1.0;
        } else if value < 0 {
            negative_count += 1.0;
        } else {
            zero_count += 1.0;
        }
    }

    // Calculate and print proportions with 6 decimal places
    println!("{:.6}", positive_count / total);
    println!("{:.6}", negative_count / total);
    println!("{:.6}", zero_count / total);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let _ = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap(); // Read and discard the first line as we don't need it

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
