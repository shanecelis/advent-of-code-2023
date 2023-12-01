use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use trie_rs::{TrieBuilder};

fn convert(num: &str) -> u32 {
    match num {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("cannot convert `{}`", num)
    }
}

fn main() {
    let mut builder = TrieBuilder::new();
    // builder.push("zero");
    builder.push("one");
    builder.push("two");
    builder.push("three");
    builder.push("four");
    builder.push("five");
    builder.push("six");
    builder.push("seven");
    builder.push("eight");
    builder.push("nine");
    let trie = builder.build();
    // let lookup: String = String::new();
    let mut accum: u32 = 0;
    let args: Vec<String> = env::args().collect();
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(l) = line {
                let mut first: Option<u32> = None;
                let mut last: Option<u32> = None;
                let mut start: usize = 0;
                let mut end: usize = 1;
                for c in l.chars() {
                    // let spelled_maybe = trie.exact_match(&l[start..end]).then_some(convert(&l[start..end]));
                    let spelled_maybe = if trie.exact_match(&l[start..end]) {
                        let result = Some(convert(&l[start..end]));
                        // start = end;
                        result
                    } else {
                        None
                    };
                    while start < end && trie.predictive_search(&l[start..end]).len() == 0 {
                        start += 1;
                    }
                    if let Some(num) = spelled_maybe.or(c.to_digit(10)) {
                        if first.is_none() {
                            first = Some(num);
                            last = Some(num);
                        } else {
                            last = Some(num);
                        }
                    }
                    end += 1;
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
