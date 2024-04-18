/*
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
*/
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'lonelyinteger' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY a as parameter.
 */

fn lonelyinteger(a: &[i32]) -> i32 {
    a.iter().fold(0, |acc, &x| acc ^ x)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap(); // `_n` is unused

    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let result = lonelyinteger(&a);

    writeln!(&mut fptr, "{}", result).ok();
}
