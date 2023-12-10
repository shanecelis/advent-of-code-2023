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

struct Grid {
    tiles: Vec<String>,
    // counts: Vec<Vec<Option<u32>>>
}

type Location = (i32, i32);

#[derive(Clone)]
struct Dir(i32, i32);

#[derive(Clone)]
struct Heading { loc: Location, dir: Dir }

impl Grid {
    fn find(&self, x: char) -> Option<Location> {
        for i in 0..self.tiles.len() {
            let mut j = 0;
            for c in self.tiles[i].chars() {
            // for j in self.tiles[i].len() {
                if x == c {
                    return Some((i as i32, j as i32));
                }
                j += 1;
            }
        }
        None
    }

    fn get(&self, location: &Location) -> Option<char> {
        self.tiles.get(location.0 as usize).map(|r| r.chars().nth(location.1 as usize)).flatten()
    }
    // fn count(&self, location: (usize, usize)) -> Option<&Option<u32>> {
    //     self.counts.get(location.0).and_then(|r| r.get(location.1))
    // }

    // fn count_mut(&mut self, location: (usize, usize)) -> Option<&mut Option<u32>> {
    //     self.counts.get_mut(location.0).and_then(|r| r.get_mut(location.1))
    // }

    fn try_move(&self, h: &Heading) -> Option<Heading> {
        let next_loc = (h.loc.0 + h.dir.0, h.loc.1 + h.dir.1);
        let next_char = self.get(&next_loc)?;
        // println!("{next_char}");
        let next_dir = match (h.dir.clone(), next_char)  {
            (Dir(i, 0), '|') => Some(Dir(i, 0)),
            (Dir(1, 0), 'L') => Some(Dir(0, 1)),
            (Dir(1, 0), 'J') => Some(Dir(0, -1)),
            (Dir(-1, 0), '7') => Some(Dir(0, -1)),
            (Dir(-1, 0), 'F') => Some(Dir(0, 1)),
            (Dir(0, i), '-') => Some(Dir(0, i)),
            (Dir(0, 1), '7') => Some(Dir(1, 0)),
            (Dir(0, 1), 'J') => Some(Dir(-1, 0)),
            (Dir(0, -1), 'L') => Some(Dir(-1, 0)),
            (Dir(0, -1), 'F') => Some(Dir(1, 0)),
            (d, 'S') => Some(d),
            // Dir(0, 1) => self.get(next_loc).unwrap() == '-',
            // Dir(-1, 0) => self.get(next_loc).unwrap() == '|',
            // Dir(0, -1) => self.get(next_loc).unwrap() == '-',
            _ => None
        };
        next_dir.map(|d| Heading { loc: next_loc, dir: d })
    }

    fn follow_pipe(&self, mut heading: Heading, finish: char) -> Option<u32> {
        let mut count = 0;
        while let Some(next_heading) = self.try_move(&heading) {
            count += 1;
            if self.get(&next_heading.loc).unwrap() == finish {
                return Some(count);
            }
            heading = next_heading;
        }
        None
    }
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
    if let Ok(mut lines) = read_lines(&args[1]) {

        // let mut line: String = lines.next().unwrap().unwrap();
        // let dirs = directions(&mut line.as_str()).unwrap();
        let tiles: Vec<_> = lines.map(|l| l.unwrap()).collect();
        let m = tiles.len();
        let n = tiles[0].len();
        let mut grid = Grid { tiles,
                          // counts: vec![vec![None; n]; m]
        };

        let start = grid.find('S').unwrap();
        for dir in [Dir(1, 0), Dir(-1, 0), Dir(0, 1), Dir(0, -1)] {
            let heading = Heading { loc: start, dir };
            if let Some(count) = grid.follow_pipe(heading, 'S') {
                println!("{}", count/2);
                return;
            }
        }
        // let start_count = grid.count_mut(start).unwrap();
        // *start_count = Some(0);

        // for line in lines {
        //     if let Ok(l) = line {

        //         // let n = node(&mut l.as_str()).unwrap();
        //         // nodes.insert(n.name.clone(), n);
        //     }
        // }
        // println!("{}");
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
