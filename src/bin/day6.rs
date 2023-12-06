use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::ops::Range;
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;
use range_ext::{self, intersect::{Intersect, IntersectionExt::*}};

#[derive(Debug, Clone, Default)]
struct Race {
    time: u32,
    distance: u32,
}

fn times(input: &mut &str) -> PResult<Vec<u32>> {
    let _ = "Time:".parse_next(input)?;
    let _ = multiple_space(input)?;
    number_list(input)
}

fn distances(input: &mut &str) -> PResult<Vec<u32>> {
    let _ = "Distance:".parse_next(input)?;
    let _ = multiple_space(input)?;
    number_list(input)
}

impl Race {
    fn eval(&self, hold: u32) -> u32 {
        assert!(hold <= self.time);
        hold * (self.time - hold)
    }

    fn ways_to_win(&self) -> u32 {
        let mut count = 0;
        for hold in 0..self.time {
            if self.eval(hold) > self.distance {
                count += 1;
            }
        }
        count
    }
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
    let args: Vec<String> = env::args().collect();
    if let Ok(mut lines) = read_lines(&args[1]) {

        let mut line: String = lines.next().unwrap().unwrap();
        let times = times(&mut line.as_str()).unwrap();
        line = lines.next().unwrap().unwrap();
        let distances = distances(&mut line.as_str()).unwrap();
        let races: Vec<Race> = std::iter::zip(times, distances).map(|(a, b)| Race { time: a, distance: b }).collect();
        let answer: u32 = races.into_iter().map(|r| r.ways_to_win()).product();
        println!("{answer}");
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

    #[test]
    fn test_ways_to_win() {
        let race = Race { time: 7, distance: 9 };
        assert_eq!(race.ways_to_win(), 4);
        let race = Race { time: 15, distance: 40 };
        assert_eq!(race.ways_to_win(), 8);
        let race = Race { time: 30, distance: 200 };
        assert_eq!(race.ways_to_win(), 9);
    }

}
