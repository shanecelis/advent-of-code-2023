use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;

#[derive(Debug, Clone, Default)]
struct Card {
    id: usize,
    winning: HashSet<u32>,
    numbers: Vec<u32>
}

impl std::str::FromStr for Card {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        card.parse(input)
            .map_err(|e| e.to_string())
    }
}

impl Card {
    fn matches(&self) -> usize {
        let mut count: usize = 0;
        for number in &self.numbers {
            if self.winning.contains(number) {
                count += 1;
            }
        }
        count
    }

    fn value(&self) -> u32 {
        match self.matches() {
            0 => 0,
            n => 1 << n - 1
        }
    }
}


fn card(input: &mut &str) -> PResult<Card> {
    let _ = "Card".parse_next(input)?;
    let _ = multiple_space.parse_next(input)?;
    let id = number.parse_next(input)? as usize;
    let _ = ":".parse_next(input)?;
    let _ = multiple_space.parse_next(input)?;
    let mut winning = HashSet::new();
    winning.extend(number_list.parse_next(input)?);
    let _ = " |".parse_next(input)?;
    let _ = multiple_space.parse_next(input)?;
    let numbers = number_list.parse_next(input)?;
    let _ = eof.parse_next(input)?;
    Ok(Card { id, winning, numbers })
}

fn number(input: &mut &str) -> PResult<u32> {
    take_while(0.., |c: char| c.is_ascii_digit())
        .try_map(|input| u32::from_str_radix(input, 10))
        .parse_next(input)
}

fn multiple_space(input: &mut &str) -> PResult<()> {
    repeat(1.., ' ')
        .parse_next(input)
}

fn number_list(input: &mut &str) -> PResult<Vec<u32>> {
    separated(0.., number, multiple_space)
        .parse_next(input)
}




fn main() {
    // let mut accum: u32 = 0;
    let args: Vec<String> = env::args().collect();
    let mut copies: Vec<u32> = Vec::new();
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(l) = line {
                let g = l.parse::<Card>().unwrap();
                while g.id >= copies.len() {
                    copies.push(0);
                }
                copies[g.id] += 1;
                for i in 1..=g.matches() {
                    while g.id + i >= copies.len() {
                        copies.push(0);
                    }
                    copies[g.id + i] += copies[g.id];
                }
            }
        }
    }
    // for c in copies.iter().enumerate() {
    //     println!("{} {}", c.0, c.1);
    // }
    let accum: u32 = copies.into_iter().sum();
    println!("{}", accum);
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


    #[test]
    fn test_card() {
        let mut input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let g = card.parse_next(&mut input).unwrap();
        assert_eq!(g.id, 1);
        assert_eq!(g.winning.len(), 5);
        assert_eq!(g.numbers.len(), 8);
        assert_eq!(g.matches(), 4);
        assert_eq!(g.value(), 8);
    }
}
