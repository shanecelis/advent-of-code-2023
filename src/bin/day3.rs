use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Debug;
// use std::ascii::Char;
use winnow::prelude::*;
use winnow::token::*;
use winnow::combinator::*;
use winnow::ascii::*;
use winnow::{Located, stream::Location};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Item {
    Period,
    Symbol(char),
    PartNumber(u32)
}
#[derive(Debug, Clone, Copy, PartialEq)]
struct Loc { column: usize, item: Item }

fn number(input: &mut Located<&str>) -> PResult<Loc> {
    let column = input.location();
    take_while(0.., |c: char| c.is_ascii_digit())
        .try_map(|input| u32::from_str_radix(input, 10).map(|n| Loc { column, item: Item::PartNumber(n) }))
        .parse_next(input)
}

fn symbol(input: &mut Located<&str>) -> PResult<Loc> {
    let column = input.location();
    none_of(['.']).map(|input| Loc { column, item: Item::Symbol(input) })
        .parse_next(input)
}

fn period(input: &mut Located<&str>) -> PResult<Loc> {
    let column = input.location();
    '.'.map(|_input| Loc { column, item: Item::Period })
        .parse_next(input)
}

fn row(input: &mut Located<&str>) -> PResult<Vec<Loc>> {
    repeat(0.., alt((period,
         number,
         symbol)))
        .parse_next(input)
}

fn has_symbol(num: &Loc, context: &[Vec<Loc>]) -> bool {
    if let Item::PartNumber(n) = num.item {
        let length = n.ilog10() + 1;
        for row in context {
            for item in row {
                if let Item::Symbol(_) = item.item {
                    // println!("col {} n {} length {} num.col {}", item.column, n, length, num.column);
                    if item.column >= num.column.saturating_sub(1) && item.column < num.column + length as usize + 1 {
                        return true;
                    }
                }
            }
        }
    } else {
        panic!();
    }
    false
}

fn sum_part_numbers(rows: Vec<Vec<Loc>>) -> u32 {
    let mut accum: u32 = 0;
    for i in 0..rows.len() {
        for item in &rows[i] {
            if let Item::PartNumber(n) = item.item {
                if has_symbol(&item, &rows[i.saturating_sub(1)..=(i+1).min(rows.len() - 1)]) {
                    // println!("found {}", n);
                    accum += n;
                }
            }
        }
    }
    accum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rows: Vec<Vec<Loc>> = Vec::new();
    if let Ok(lines) = read_lines(&args[1]) {
        for line in lines {
            if let Ok(l) = line {
                let r = row.parse(Located::new(&l)).unwrap();
                // println!("r {}", r.len());
                let filtered: Vec<_> = r.into_iter().filter(|loc| loc.item != Item::Period).collect();
                // println!("filtered {}", filtered.len());
                rows.push(filtered);
            }
        }
    }
    let accum = sum_part_numbers(rows);
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
    fn test_number_located() {
        let mut input = Located::new("12");

        assert_eq!(input.location(), 0);
        let n = number.parse_next(&mut input).unwrap();
        assert_eq!(input.location(), 2);
        assert_eq!(n, Loc { column: 0, item: Item::PartNumber(12) });
    }
}
