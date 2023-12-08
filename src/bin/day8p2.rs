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

#[derive(Debug,Clone)]
enum Dir {
    Left,
    Right
}

#[derive(Debug,Clone)]
struct Node<T> {
    name: T,
    directions: (T, T)
}

impl<T> Node<T> {

    fn map<F,U>(self, f: F) -> Node<U>
        where F: Fn(T) -> U,
    U: Hash + Eq
    {
        Node { name: f(self.name),
               directions: (f(self.directions.0),
                            f(self.directions.1))
        }
    }
}

struct State<T>
where T: Eq + Hash + PartialEq
{
    locations: Vec<T>,
    end_nodes: HashSet<T>,
    nodes: HashMap<T, Node<T>>,
}

impl<T> State<T>
where T: Eq + Hash + PartialEq + Clone
{
    fn go(&mut self, d: Dir) {
        for location in &mut self.locations {
            let dirs = &self.nodes.get(location).unwrap().directions;
            *location = match d {
                Dir::Left => dirs.0.clone(),
                Dir::Right => dirs.1.clone(),
            }
        }
    }

    fn is_done(&self) -> bool {
        // self.locations.iter().all(|n| n.chars().last().unwrap() == 'Z')
        self.locations.iter().all(|n| self.end_nodes.contains(n))
    }

    fn map<F,U>(self, f: F) -> State<U>
        where F: Fn(T) -> U,
    U: Hash + Eq
    {
        State {
            locations: self.locations.into_iter().map(&f).collect(),
            end_nodes: self.end_nodes.into_iter().map(&f).collect(),
            nodes: self.nodes.into_values().map(|x| (f(x.name.clone()), x.map(&f))).collect()
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

fn node(input: &mut &str) -> PResult<Node<String>> {
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
        let mut nodes: HashMap<String, Node<String>> = HashMap::new();

        for line in lines {
            if let Ok(l) = line {
                let n = node(&mut l.as_str()).unwrap();
                nodes.insert(n.name.clone(), n);
            }
        }
        let mut state: State<String> = State { locations: nodes.keys().filter(|n| n.chars().last().unwrap() == 'A').cloned().collect(),
                                        end_nodes: nodes.keys().filter(|n| n.chars().last().unwrap() == 'Z').cloned().collect(),
                                        nodes: nodes };
        let mut state = state.map(my_hash);

        let mut count = 0;
        for d in dirs.into_iter().cycle() {
            if state.is_done() {
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
