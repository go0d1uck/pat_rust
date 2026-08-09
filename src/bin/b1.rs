use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, stdin},
};

fn main() -> io::Result<()> {
    let mut input: Box<dyn BufRead> = if cfg!(debug_assertions) {
        Box::new(BufReader::new(File::open("input")?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    println!("{:}", content);
    Ok(())
}
