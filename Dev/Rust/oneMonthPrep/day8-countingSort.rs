use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'countingSort' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn countingSort(arr: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new(); //The original idea is to create a hashmap where the value stores the number and the key the occurance, then we can iterate through it to get the sorted array.
    
    
    for &number in arr {
        *counts.entry(number).or_insert(0) += 1;
    }

    
    let mut frequency = vec![0; 100];
    for (key, value) in counts {
        if key >= 0 && key < 100 {
            frequency[key as usize] = value;
            println!("The key {} and value {}", key, value);
        }
    }

    frequency
}
// fn countingSort(arr: &[i32]) -> Vec<i32> {
//     let mut frequency = vec![0; 100]; // Initializes a frequency array with 100 zeros

//     // Count each number's frequency
//     for &number in arr {
//         frequency[number as usize] += 1; // Increment the frequency for each number
//     }

//     frequency
// } // This version is a lot prettier tbh...

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = countingSort(&arr);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
