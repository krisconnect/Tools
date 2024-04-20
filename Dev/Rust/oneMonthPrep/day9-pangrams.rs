use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::collections::HashSet;

/*
 * Complete the 'pangrams' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn pangrams(s: &str) -> String {
    // Originally I wanted to go the C way and create a hashmap for ascii characters and return pangram or not pangram based on the boolean switch if all ascii codes are at least 1 in the hashmap, but Rust can do this in a lot more sophisticated way with hashset. Pretty much the same thing though. https://doc.rust-lang.org/std/collections/struct.HashSet.html
    let mut letter_set: HashSet<char> = HashSet::new();  // Use a HashSet to track unique letters

    
    for c in s.to_lowercase().chars() {
        if c.is_alphabetic() {
            letter_set.insert(c);
        }
    }

    
    if letter_set.len() == 26 {
        "pangram".to_string()
    } else {
        "not pangram".to_string()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = pangrams(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
