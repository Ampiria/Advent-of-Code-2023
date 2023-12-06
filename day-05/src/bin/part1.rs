use std::collections::BTreeMap;
use nom::{IResult, sequence::separated_pair, bytes::complete::{tag, is_not}};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Range {
    start: u128,
    length: u128
}

#[derive(Debug)]
struct ResourceMapRange <'a> {
    dst: &'a str,
    range_query: RangeQueryer,
}

#[derive(Debug)]
struct RangeQueryer {
    src_ranges: Vec<Range>,
    dst_ranges: Vec<Range>
}

impl RangeQueryer {
    fn new(src_ranges: Vec<Range>, dst_ranges: Vec<Range>) -> Self{
        RangeQueryer {
            src_ranges, 
            dst_ranges 
        }
    }

    fn add_mapping(&mut self, src: Range, dst: Range) {
        self.src_ranges.push(src);
        self.dst_ranges.push(dst);
    }

    fn query(&self, trg: u128) -> u128{
        let mut result = trg;
        for (i,r) in self.src_ranges.iter().enumerate() {
            match r.contains(trg) {
                Some(offset) => result = self.dst_ranges.get(i).expect("invalid mapping").map_target(offset),
                None => ()
            };
        };
        result
    }
}


impl Range {
    fn contains(&self, target: u128) -> Option<u128> {
        if self.start <= target && target < self.start + self.length {
            Some(target - self.start)
        } else {
            None
        }
    }

    fn map_target(&self, offset: u128) -> u128 {
        if offset > self.length {
            panic!("invalid offset")
        }
        self.start + offset
    }

    fn new(start: u128, length: u128) -> Self {
        Range { 
            start,
            length 
        }
    }
}

impl <'a> ResourceMapRange<'a> {
    fn new(dst: &'a str) -> Self {
        ResourceMapRange {
            dst, 
            range_query: RangeQueryer::new(Vec::new(), Vec::new())
        }
    }
}

fn parse_seeds(line: &str) -> IResult<&str, &str> {
    tag("seeds: ")(line)
}


fn ranges_parser(line: &str) -> (Range, Range) {
    let mut nums = line.split_whitespace().map(|x| x.parse::<u128>().expect("invalid ranges"));
    let dst_start = nums.next().expect("no src_start");
    let src_start = nums.next().expect("no src_start");
    let length = nums.next().expect("no src_start");
    (Range::new(src_start, length), Range::new(dst_start, length))
}

fn parse_description(line: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(is_not("-"), tag("-to-"), is_not(" \t\r\n"))(line)
}

fn part1(input: &str) -> u128 {
    let mut lines = input.lines();
    let mut maps = BTreeMap::new();

    let mut seeds: Vec<u128> = match parse_seeds(lines.next().expect("empty input")) {
                              Ok((nums, _)) => nums.split_whitespace().map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>(),
                              Err(_) => panic!("didn't start with seeds")
                          };

    let mut key = "";
    lines.filter(|line| line.trim().len() > 0).for_each(|line| {
        match parse_description(line.trim()) {
            Ok((_, (src, dst))) => {maps.insert(src, ResourceMapRange::new(dst));
                                                key = src;},
            Err(_) => {
                        let (src, dst) = ranges_parser(line); 
                        maps.get_mut(key).unwrap().range_query.add_mapping(src, dst);
                      }
        };
    });
    
    let mut k = "seed";
    while !k.eq("location") {
        let mapping = maps.get(k).expect(&format!("invalid key: {:?}", k));
        seeds = seeds.iter().map(|x| mapping.range_query.query(*x)).collect();
        k = mapping.dst;
    }
    
    *seeds.iter().min().expect("no minimum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4");
        assert_eq!(result, 35);
    }

}


