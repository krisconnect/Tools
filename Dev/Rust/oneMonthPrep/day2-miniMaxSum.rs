use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i64]) {
    let total_sum: i64 = arr.iter().sum(); // Calculate total sum of all elements
    let mut min_sum = i64::MAX;
    let mut max_sum = i64::MIN;

    for &value in arr {
        let sum_excluding_current = total_sum - value; // Calculate sum excluding the current element
        //println!("Current iteration {}, value of sum_excluding_current {}", value, sum_excluding_current);
        if sum_excluding_current > max_sum {
            max_sum = sum_excluding_current; // Update maximum sum
        }
        //println!("Current iteration {}, value of max_sum {}", value, max_sum);
        if sum_excluding_current < min_sum {
            min_sum = sum_excluding_current; // Update minimum sum
        }
        //println!("Current iteration {}, value of min_sum {}", value, min_sum);
    }

    println!("{} {}", min_sum, max_sum);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i64> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap()) // Parse integers as i64
        .collect();

    miniMaxSum(&arr);
}
