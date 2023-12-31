use std::env;
use std::cmp::Ordering::{self, *};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;

#[derive(Debug,Clone)]
enum Dir {
    Left,
    Right
}

#[derive(Debug,Clone)]
struct Node {
    name: String,
    directions: (String, String)
}

struct State {
    location: String,
    nodes: HashMap<String, Node>,
}

impl State {
    fn go(&mut self, d: Dir) {
        let dirs = &self.nodes.get(&self.location).unwrap().directions;
        self.location = match d {
            Dir::Left => dirs.0.clone(),
            Dir::Right => dirs.1.clone(),
        }
    }
}


fn dir(input: &mut &str) -> PResult<Dir> {
    alt(('L'.value(Dir::Left),
         'R'.value(Dir::Right))).parse_next(input)
}

fn directions(input: &mut &str) -> PResult<Vec<Dir>> {
    repeat(1.., dir).parse_next(input)
}

fn label(input: &mut &str) -> PResult<String> {
    take_while(1.., ('A'..='Z'))
        .map(String::from)
        .parse_next(input)
}

fn node(input: &mut &str) -> PResult<Node> {
    let name = label(input)?;
    let _ = " = (".parse_next(input)?;
    let left = label(input)?;
    let _ = ", ".parse_next(input)?;
    let right = label(input)?;
    let _ = ")".parse_next(input)?;
    Ok(Node { name, directions: (left, right) })
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

        let mut line: String = lines.next().unwrap().unwrap();
        let dirs = directions(&mut line.as_str()).unwrap();
        let _ = lines.next();
        let mut nodes: HashMap<String, Node> = HashMap::new();

        for line in lines {
            if let Ok(l) = line {
                let n = node(&mut l.as_str()).unwrap();
                nodes.insert(n.name.clone(), n);
            }
        }
        let mut state = State { location: "AAA".into(),
                                nodes: nodes };

        let mut count = 0;
        for d in dirs.into_iter().cycle() {
            if state.location == "ZZZ" {
                break;
            }
            state.go(d);
            count += 1;
        }
         println!("{count}");
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
    fn test_node() {
        let mut input = "AAA = (BBB, CCC)";

        let n = node.parse_next(&mut input).unwrap();
        assert_eq!(n.name, "AAA".to_string());
        assert_eq!(n.directions.0, "BBB".to_string());
        assert_eq!(n.directions.1, "CCC".to_string());
    }
}
