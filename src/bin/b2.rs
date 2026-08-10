use std::fs::File;
#[cfg(not(debug_assertions))]
use std::io::stdin;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    let mut input = Box::new(BufReader::new(File::open("input")?));

    #[cfg(not(debug_assertions))]
    let mut input = Box::new(BufReader::new(stdin()));

    let mut n = String::new();

    input.read_line(&mut n)?;

    let m: u32 = n
        .trim()
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .sum();

    let n2s = [
        "ling", "yi", "er", "san", "si", "wu", "liu", "qi", "ba", "jiu",
    ];

    println!(
        "{:}",
        m.to_string()
            .chars()
            .map(|c| n2s[c.to_digit(10).unwrap() as usize])
            .fold(String::new(), |pre, next| pre + " " + next)
            .trim()
    );
    Ok(())
}
