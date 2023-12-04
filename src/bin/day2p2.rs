use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;
use winnow::ascii::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Blue,
    Red,
    Green
}

#[derive(Debug, Clone, Default)]
struct Game {
    id: u32,
    sets: Vec<Vec<Count>>
}

#[derive(Debug, Clone, Default, PartialEq)]
struct Count(u32,  u32,  u32);

impl std::str::FromStr for Game {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        game.parse(input)
            .map_err(|e| e.to_string())
    }
}


fn game(input: &mut &str) -> PResult<Game> {
    let _ = "Game ".parse_next(input)?;
    let id = number.parse_next(input)?;
    let _ = ": ".parse_next(input)?;
    let sets = sets.parse_next(input)?;
    let _ = eof.parse_next(input)?;
    Ok(Game { id, sets })
}

fn number(input: &mut &str) -> PResult<u32> {
    take_while(0.., |c: char| c.is_ascii_digit())
        .try_map(|input| u32::from_str_radix(input, 10))
        .parse_next(input)
}

fn pair(input: &mut &str) -> PResult<Count> {
    let n = number.parse_next(input)?;
    let _ = ' '.parse_next(input)?;
    let c = color.parse_next(input)?;
    Ok(match c {
        Color::Red => Count(n, 0, 0),
        Color::Green => Count(0, n, 0),
        Color::Blue => Count(0, 0, n),
    })
}

fn color(input: &mut &str) -> PResult<Color> {
    alt(
    ("blue".value(Color::Blue),
     "red".value(Color::Red),
     "green".value(Color::Green),
    )).parse_next(input)
}

fn set(input: &mut &str) -> PResult<Vec<Count>> {
    separated(0.., pair, ", ").parse_next(input)
}

fn sets(input: &mut &str) -> PResult<Vec<Vec<Count>>> {
    separated(0.., set, "; ").parse_next(input)
}

fn is_valid(game: &Game, red_limit: u32, green_limit: u32, blue_limit: u32) -> bool {
    for set in &game.sets {
        for Count(r, g, b) in set {
            if r > &red_limit || g > &green_limit || b > &blue_limit {
                return false;
            }
        }
    }
    true
}

fn find_min(game: &Game) -> Count {
    let mut c = Count(0, 0, 0);

    for set in &game.sets {
        for Count(r, g, b) in set {
            if *r > c.0 {
                c.0 = *r;
            }
            if *g > c.1 {
                c.1 = *g;
            }

            if *b > c.2 {
                c.2 = *b;
            }
        }
    }
    c
}

fn main() {
    let mut accum: u32 = 0;
    let args: Vec<String> = env::args().collect();
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(l) = line {
                let g = l.parse::<Game>().unwrap();
                let c = find_min(&g);
                accum += c.0 * c.1 * c.2;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        let mut input = "blue";

        let c = color.parse_next(&mut input).unwrap();
        assert_eq!(c, Color::Blue);
    }

    #[test]
    fn test_number() {
        let mut input = "12";

        let n = number.parse_next(&mut input).unwrap();
        assert_eq!(n, 12);
    }

    #[test]
    fn test_pair() {
        let mut input = "12 blue, 1 red";

        let p = pair.parse_next(&mut input).unwrap();
        assert_eq!(Count(0,0,12), p);
        // assert_eq!(p.1, Color::Blue);
    }

    #[test]
    fn test_list() {
        let mut input = "12 blue, 1 red";

        let l = set.parse_next(&mut input).unwrap();
        assert_eq!(l[1].0, 1);
        assert_eq!(l.len(), 2);
    }

    #[test]
    fn test_sets() {
        let mut input = "12 blue, 1 red; 1 green";

        let l = sets.parse_next(&mut input).unwrap();
        assert_eq!(l[0][1].0, 1);
        assert_eq!(l.len(), 2);
        assert_eq!(l[1].len(), 1);
        assert_eq!(l[0].len(), 2);
    }

    #[test]
    fn test_game() {
        let mut input = "Game 3: 12 blue, 1 red";

        let g = game.parse_next(&mut input).unwrap();
        assert_eq!(g.id, 3);
        assert_eq!(g.sets.len(), 1);
        assert_eq!(g.sets[0].len(), 2);
    }
}
