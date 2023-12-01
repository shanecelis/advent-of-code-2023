use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut accum: u32 = 0;
    if let Ok(lines) = read_lines("day1.txt") {
        for line in lines {
            if let Ok(l) = line {
                let mut first: Option<u32> = None;
                let mut last: Option<u32> = None;
                for c in l.chars() {
                    if let Some(num) = c.to_digit(10) {
                        if first.is_none() {
                            first = Some(num);
                            last = Some(num);
                        } else {
                            last = Some(num);
                        }
                    }
                }
                accum += first.unwrap() * 10 + last.unwrap();
            }
        }
    }
    println!("{}", accum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
