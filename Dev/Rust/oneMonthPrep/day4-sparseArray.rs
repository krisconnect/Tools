use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'matchingStrings' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. STRING_ARRAY strings
 *  2. STRING_ARRAY queries
 */

fn matchingStrings(strings: &[String], queries: &[String]) -> Vec<i32> {
    let mut string_counts = HashMap::new(); //Create a new hashmap that will store strings with the number of times they are in the provided strings.
    for string in strings {
        *string_counts.entry(string).or_insert(0) += 1;
    } //fill up the hashmap with the sting and value 0, or if already exists add 1 to it.
    queries.iter().map(|query| *string_counts.get(query).unwrap_or(&0) as i32).collect() // Return the number of string occurance stored in the hashmap
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let strings_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut strings: Vec<String> = Vec::with_capacity(strings_count as usize);

    for _ in 0..strings_count {
        let strings_item = stdin_iterator.next().unwrap().unwrap();
        strings.push(strings_item);
    }

    let queries_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut queries: Vec<String> = Vec::with_capacity(queries_count as usize);

    for _ in 0..queries_count {
        let queries_item = stdin_iterator.next().unwrap().unwrap();
        queries.push(queries_item);
    }

    let res = matchingStrings(&strings, &queries);

    for i in 0..res.len() {
        write!(&mut fptr, "{}", res[i]).ok();

        if i != res.len() - 1 {
            writeln!(&mut fptr).ok();
        }
    }

    writeln!(&mut fptr).ok();
}
