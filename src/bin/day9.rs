use std::env;
use std::cmp::Ordering::{self, *};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;

#[derive(Debug)]
struct Game {
    entries: Vec<Vec<i64>>
}

impl Game {
    fn next(&mut self) -> bool {
        println!("{:?}", &self.entries.last().unwrap());
        let l: Vec<_> = self.entries.last().unwrap().windows(2).map(|x| x[1] - x[0]).collect();
        // println!("{:?}", &l);
        let is_done = l.iter().all(|x| x == &0);
        self.entries.push(l);
        is_done
    }

    fn fill_next(&mut self) -> Result<i64, ()> {
        let n = self.entries.len();
        self.entries.last_mut().ok_or(())?.push(0);
        for i in 1..n {
            println!("i {i} n {n}");
            let l = self.entries[n - 1 - (i - 1)].last().copied().ok_or(())?;
            println!("a");
            let upper = &mut self.entries[n - 1 - i];
            println!("a");
            let u = upper.last().copied().ok_or(())?;
            // u - u* == l
            let u_p = u + l;
            upper.push(u_p);
        }
        self.entries[0].last().copied().ok_or(())
    }
}


fn game(input: &mut &str) -> PResult<Game> {
    let numbers = number_list(input)?;
    Ok(Game { entries: vec![numbers] })
}

fn label(input: &mut &str) -> PResult<String> {
    take_while(1.., ('0'..='9','A'..='Z'))
        .map(String::from)

        .parse_next(input)
}

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}

fn dash(input: &mut &str) -> PResult<char> {
    '-'.parse_next(input)
}

fn number(input: &mut &str) -> PResult<i64> {
    let negative = dash(input).is_ok();
    take_while(0.., |c: char| c.is_ascii_digit())
        .try_map(|input| i64::from_str_radix(input, 10).map(|x| x * (if negative { -1 } else { 1 })))
        .parse_next(input)
}

fn multiple_space(input: &mut &str) -> PResult<()> {
    repeat(1.., ' ')
        .parse_next(input)
}

fn number_list(input: &mut &str) -> PResult<Vec<i64>> {
    separated(0.., number, multiple_space)
        .parse_next(input)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(mut lines) = read_lines(&args[1]) {

        let answer: i64 = 0;
        // let mut line: String = lines.next().unwrap().unwrap();
        // let dirs = directions(&mut line.as_str()).unwrap();
        let mut games: Vec<Game> = Vec::new();

        for line in lines {
            if let Ok(l) = line {
                let g = game(&mut l.as_str()).unwrap();
                games.push(g);
                // let n = node(&mut l.as_str()).unwrap();
                // nodes.insert(n.name.clone(), n);
            }
        }
        let mut accum: i64 = 0;
        for game in &mut games {
            while ! game.next() {
            }
            let a = match game.fill_next() {
                Ok(x) => x,
                Err(_) => panic!("Problem with game {:?}", game)

            };
            println!("a {a}");
            accum += a;

        }
        println!("{accum}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let mut input = "12";

        let n = number.parse_next(&mut input).unwrap();
        assert_eq!(n, 12);
    }

}
