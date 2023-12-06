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
struct Map {
    from: String,
    to: String,
    ranges: Vec<MapRange>
}

#[derive(Debug, Clone, Default)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    length: u64,
}

// #[derive(Debug)]
// struct CompositeRange {
//     ranges: Vec<Range<u64>>
// }

// impl CompositeRange {

//     fn select_mut<'a>(&'a mut self, other: &Range<u64>) -> Vec<usize> {
//         let mut result: Vec<usize> = Vec::new();
//         for i in 0..self.ranges.len() {
//             match self.ranges[i].intersect_ext(other) {
//                 Empty | Less | Greater => {},
//                 Same | Over => result.push(i),
//                 LessOverlap => {
//                     self.ranges.insert(i + 1, Range { start: other.start, end: self.ranges[i].end });
//                     self.ranges[i].end = other.start - 1;
//                     result.push(i + 1);
//                 }
//                 GreaterOverlap => {
//                     self.ranges.insert(i + 1, Range { start: other.end, end: self.ranges[i].end });
//                     self.ranges[i].end = other.end;
//                     result.push(i);
//                 }
//                 Within => {
//                     let end = self.ranges[i].end;
//                     self.ranges[i].end = other.start - 1;
//                     self.ranges.insert(i + 1, other);
//                     self.ranges.insert(i + 2, other);


//                 }
//             }
//         }
//         result
//     }
// }

impl std::str::FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        map_header.parse(input)
            .map_err(|e| e.to_string())
    }
}

impl std::str::FromStr for MapRange {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        map_range.parse(input)
            .map_err(|e| e.to_string())
    }
}

fn seeds(input: &mut &str) -> PResult<Vec<u64>> {
    let _ = "seeds: ".parse_next(input)?;
    number_list(input)
}

impl MapRange {
    fn source_range(&self) -> Range<u64> {
        self.source_start..(self.source_start + self.length)
    }

    fn dest_range(&self) -> Range<u64> {
        self.dest_start..(self.dest_start + self.length)
    }

    fn covers_source(&self, source: &Range<u64>) -> Option<Range<u64>> {
        let r = self.source_range();
        match r.intersect_ext(source) {
            Empty | Less | Greater | Same => None,
            Over => Some(Range { start: source.start, end: source.end }),
            LessOverlap => Some(Range { start: source.start, end: r.end }),
            GreaterOverlap => Some(Range { start: r.start, end: source.end }),
            Within => Some(r),
        }
    }

    fn map_range(&self, source: &Range<u64>) -> Option<Range<u64>> {
        let source_map_maybe = self.covers_source(source);
        source_map_maybe.map(|s| Range { start: s.start - self.source_start + self.dest_start,
                                         end: s.end - self.source_start + self.dest_start })

    }

    fn map(&self, source: u64) -> Option<u64> {
        let range = self.source_start..(self.source_start + self.length);
        if range.contains(&source) {
            Some(source - self.source_start + self.dest_start)
        } else {
            None
        }
    }
}

impl Map {
    fn sort(&mut self) {
        self.ranges.sort_by(|a, b| a.source_start.cmp(&b.source_start));
    }

    fn map(&self, source: u64) -> u64 {
        for map_range in &self.ranges {
            if let Some(x) = map_range.map(source) {
                return x;
            }
        }
        return source;
    }

    fn map_range<'a>(&'a self, source: &'a Range<u64>) -> impl Iterator<Item = Range<u64>> + 'a {
        let mut result: Vec<Range<u64>> = Vec::new();
        let mut start = source.start;
        let mut end = source.end;
        for range in &self.ranges {
            if let Some(source_range) = range.covers_source(source) {
                result.push(Range { start: start, end: source_range.start });

            }

        }
        self.ranges.iter().filter_map(|r| r.map_range(source))
    }
}

fn map_range(input: &mut &str) -> PResult<MapRange> {
    let dest_start = number.parse_next(input)?;
    let _ = " ".parse_next(input)?;
    let source_start = number.parse_next(input)?;
    let _ = " ".parse_next(input)?;
    let length = number.parse_next(input)?;
    Ok(MapRange { dest_start, source_start, length })
}

fn map_header(input: &mut &str) -> PResult<Map> {
    let from = map_source(input)?;
    let _ = "-to-".parse_next(input)?;
    let to = map_source(input)?;
    let _ = " map:".parse_next(input)?;
    Ok(Map { from, to, ranges: Vec::new() })
}

fn map_source(input: &mut &str) -> PResult<String> {
    take_while(0.., |c:char| c != '-' && c != ' ')
        .map(|input| String::from(input))
        .parse_next(input)
}

fn number(input: &mut &str) -> PResult<u64> {
    take_while(0.., |c: char| c.is_ascii_digit())
        .try_map(|input| u64::from_str_radix(input, 10))
        .parse_next(input)
}

fn multiple_space(input: &mut &str) -> PResult<()> {
    repeat(1.., ' ')
        .parse_next(input)
}

fn number_list(input: &mut &str) -> PResult<Vec<u64>> {
    separated(0.., number, multiple_space)
        .parse_next(input)
}

fn eval_maps(seed: u64, maps: &Vec<Map>) -> u64 {
    let mut value: u64 = seed;
    for map in maps {
        value = map.map(value);
    }
    value
}

fn find_range_maps(seed: Range<u64>, maps: &Vec<Map>) -> impl Iterator<Item = Range<u64>> {
    let mut value: Vec<Range<u64>> = vec![seed];
    for map in maps {
        let mut new_value: Vec<Range<u64>> = Vec::new();
        for v in &value {
            new_value.extend(map.map_range(v));
        }
        value.extend(new_value);
    }
    value.into_iter()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut maps: Vec<Map> = Vec::new();
    let mut current_map: Option<Map> = None;
    if let Ok(mut lines) = read_lines(&args[1]) {
        let first_line: String = lines.next().unwrap().unwrap();
        let seeds = seeds(&mut first_line.as_str()).unwrap();
        println!("seeds: {:?}", seeds);
        for line in lines {
            if let Ok(l) = line {
                if l.trim().len() == 0 {
                    if let Some(map) = current_map.take() {
                        maps.push(map);
                    }
                } else {
                    match current_map {
                        None => current_map = Some(l.parse::<Map>().unwrap()),
                        Some(ref mut map) => map.ranges.push(l.parse::<MapRange>().unwrap()),
                    }
                }
            }
        }
        if let Some(map) = current_map.take() {
            maps.push(map);
        }
        for map in &mut maps {
            map.sort();
        }
        let mut lowest_location: u64 = u64::MAX;
        for i in 0..(seeds.len()/2) {
            for seed in seeds[i*2]..(seeds[i*2] + seeds[i*2+1]) {
                let value: u64 = eval_maps(seed, &maps);
                lowest_location = value.min(lowest_location);
            }
        }
        println!("lowest_location {lowest_location}");
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
    fn test_map() {
        let mut input = "humidity-to-location map:";

        let g = map_header.parse_next(&mut input).unwrap();
        assert_eq!(&g.from, "humidity");
        assert_eq!(&g.to, "location");
    }

    #[test]
    fn test_seeds() {
        let mut input = "seeds: 79 14 55 13";

        let s = seeds.parse_next(&mut input).unwrap();
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn test_map_range() {
        let range = MapRange { dest_start: 52, source_start: 50, length: 48 };
        assert_eq!(range.map(50), Some(52));
        assert_eq!(range.map(48), None);
    }

    #[test]
    fn test_map_range_check_extreme() {
        let range = MapRange { dest_start: 52, source_start: 50, length: 48 };
        assert_eq!(range.map_range(&(0..10)), None);
        assert_eq!(range.map_range(&(0..60)), Some(52..62));
        assert_eq!(range.map_range(&(60..65)), Some(62..67));
        assert_eq!(range.map_range(&(0..200)), Some(52..100));
        assert_eq!(range.map_range(&(200..300)), None);
    }
}
