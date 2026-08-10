use std::{
    fs::File,
    io::{self, BufRead, BufReader, stdin},
    iter,
};

fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    let mut input = Box::new(BufReader::new(File::open("input")?));

    #[cfg(not(debug_assertions))]
    let mut input = Box::new(BufReader::new(stdin()));

    let mut content = String::new();
    input.read_line(&mut content)?;
    let mut n: u32 = content.trim().parse().unwrap();
    let ans = iter::from_fn(|| {
        if n == 1 {
            None
        } else {
            n = if n % 2 == 0 { n / 2 } else { (3 * n + 1) / 2 };
            Some(())
        }
    })
    .count();
    println!("{:}", ans);
    Ok(())
}
