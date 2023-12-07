use std::env;
use std::cmp::Ordering::{self, *};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

impl std::str::FromStr for Hand {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        hand.parse(input)
            .map_err(|e| e.to_string())
    }
}

impl Hand {
    fn kind(&self) -> u8 {
        let mut kinds : [u8; 15] =  [0; 15];
        for card in &self.cards {
            kinds[*card as usize] += 1;
        }
        kinds.sort();
        match (kinds[14], kinds[13]) {
            (1, _) => 1,
            (2, 1) => 2,
            (2, 2) => 3,
            (3, 1) => 4,
            (3, 2) => 5,
            (4, _) => 6,
            (5, _) => 7,
            _ => panic!("Unexpected hand {} {}", kinds[14], kinds[13])
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let h = self.kind();
        let o = other.kind();
        if h < o {
            Less
        } else if h > o {
            Greater
        } else {
            for i in 0..5 {
                if self.cards[i] < other.cards[i] {
                    return Less;
                } else if self.cards[i] > other.cards[i] {
                    return Greater;
                }
            }
            Equal
        }
    }
}

impl PartialOrd<Hand> for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn cards(input: &mut &str) -> PResult<Vec<u8>> {
    repeat(1..=5, alt(('T'.value(10),
         'J'.value(11),
         'Q'.value(12),
         'K'.value(13),
         'A'.value(14),
         take(1 as usize).map(|input| u8::from_str_radix(input, 10).unwrap()))))
        .parse_next(input)
}

fn hand(input: &mut &str) -> PResult<Hand> {
    let cards = cards(input)?;
    let _ = multiple_space(input)?;
    let bid = number(input)?;
    Ok(Hand { cards, bid })
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

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(lines) = read_lines(&args[1]) {
        let mut hands: Vec<Hand> = Vec::new();

        for line in lines {
            if let Ok(l) = line {
                hands.push(l.parse::<Hand>().unwrap());
            }
        }
        hands.sort();
        let mut accum: u32 = 0;
        for i in 0..hands.len() {
            accum += (i + 1) as u32 * hands[i].bid;
        }

        // let times = times(&mut line.as_str()).unwrap();
        // line = lines.next().unwrap().unwrap();
        // let distances = distances(&mut line.as_str()).unwrap();
        // let races: Vec<Hand> = std::iter::zip(times, distances).map(|(a, b)| Hand { time: a, distance: b }).collect();
        // println!("{races:?}");
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

    #[test]
    fn test_hand_parse() {
        let mut input = "KQJT9 1";

        let h = hand.parse_next(&mut input).unwrap();
        assert_eq!(h.cards[0], 13);
        assert_eq!(h.cards[1], 12);
        assert_eq!(h.cards[2], 11);
        assert_eq!(h.cards[3], 10);
        assert_eq!(h.cards[4], 9);
        assert_eq!(h.kind(), 1);
    }

    #[test]
    fn test_hand_kind2() {
        let mut input = "KKJT9 1";

        let h = hand.parse_next(&mut input).unwrap();
        assert_eq!(h.kind(), 2);
    }

    #[test]
    fn test_hand_kind3() {
        let mut input = "KKJJ9 1";

        let h = hand.parse_next(&mut input).unwrap();
        assert_eq!(h.kind(), 3);
    }
}
