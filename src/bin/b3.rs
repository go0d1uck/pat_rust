#[cfg(not(debug_assertions))]
use std::io::stdin;
use std::io::{BufRead, BufReader};
use std::{cmp::min, fs::File};

#[derive(Default, Debug)]
struct PosAndCnt {
    p_pos: usize,
    p_cnt: usize,
    t_pos: usize,
    t_cnt: usize,
}

fn get_p_t(str: &str) -> Option<PosAndCnt> {
    let mut ps = PosAndCnt::default();
    for (idx, val) in str.chars().enumerate() {
        match val {
            'P' => {
                ps.p_cnt += 1;
                ps.p_pos = idx;
                if ps.p_cnt > 1 {
                    return None;
                }
            }
            'T' => {
                ps.t_cnt += 1;
                ps.t_pos = idx;
                if ps.t_cnt > 1 {
                    return None;
                }
            }
            'A' => {}
            _ => return None,
        }
    }
    Some(ps)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    let mut input = Box::new(BufReader::new(File::open("input")?));

    #[cfg(not(debug_assertions))]
    let mut input = Box::new(BufReader::new(stdin()));

    let mut n = String::new();

    input.read_line(&mut n)?;
    // let mut max_t = 0;
    for _ in 0..n.trim().parse().unwrap() {
        let mut str = String::new();
        input.read_line(&mut str)?;
        str = str.trim().to_owned();
        let ps = get_p_t(&str);
        match ps {
            Some(ps) => {
                let (left, mid, right) =
                    (ps.p_pos, ps.t_pos - ps.p_pos - 1, str.len() - ps.t_pos - 1);
                if mid >= 1 && (right - left) == (mid - 1) * left {
                    println!("YES");
                } else {
                    println!("NO")
                }
            }
            None => {
                println!("NO")
            }
        }
    }
    Ok(())
}
