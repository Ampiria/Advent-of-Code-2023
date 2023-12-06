use std::collections::BTreeMap;
use nom::{IResult, error::Error, combinator::iterator, 
          sequence::{separated_pair, terminated}, bytes::complete::tag,
          character::complete::{digit1, space1, space0, alpha1}};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u128,
    end: u128,
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

#[derive(Debug)]
enum IntersectionResult {
    Contains(Range),
    Slices(Range, Slice),
    NoOverlap
}

#[derive(Debug)]
enum Slice {
    Single(Range),
    Double(Range,Range)
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

    fn query(&self, trg: &Range) -> Vec<Range>{
        use IntersectionResult::*;
        use Slice::*;
        
        let mut results = vec![];
        let mut unconsumed = vec![*trg];
        self.src_ranges.iter().zip(self.dst_ranges.iter()).for_each(|(src, dst)| {
            let mut new_unconsumed = vec![];
            unconsumed.iter().for_each(|range| {
                match src.intersection(&range) {
                    Contains(offset) => results.push(dst.map_offset(&offset)),
                    Slices(offset, Single(unmatched) ) => {
                                                            results.push(dst.map_offset(&offset)); 
                                                            new_unconsumed.push(unmatched)
                                                           },
                    Slices(offset, Double(unm1, unm2)) => {
                                                            results.push(dst.map_offset(&offset)); 
                                                            new_unconsumed.push(unm1); 
                                                            new_unconsumed.push(unm2)
                                                          },
                    NoOverlap => new_unconsumed.push(*range)
                }
            });
            unconsumed = new_unconsumed;     
        });
        results.append(&mut unconsumed);
        results
    }
}

impl Range {
    fn intersection(&self, target: &Self) -> IntersectionResult {
        use IntersectionResult::*;
        use Slice::*;
        if self.start <= target.start {
            let offset = target.start - self.start;
            if target.end <= self.end {
                Contains(Range::new(offset, target.length))
            } else if target.start < self.end{
                Slices(Range::new(offset, self.end - target.start), 
                       Single(Range::new(self.end, target.end - self.end)))
            } else {
                NoOverlap
            }
        } else if self.start < target.end {
            if self.end < target.end {
                Slices(Range::new(0, self.length), 
                       Double(Range::new(target.start, self.start - target.start),
                              Range::new(self.end , target.end - self.end)))
            } else {
                Slices(Range::new(0, target.end - self.start), 
                       Single(Range::new(target.start, self.start - target.start)))
            }
        } else {
            NoOverlap
        }
    }

    fn map_offset(&self, offset: &Range) -> Self {
        Range::new(self.start + offset.start, offset.length)
    }

    fn new(start: u128, length: u128) -> Self {
        Range { 
            start,
            end: start + length,
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

fn parse_seed_ranges(line: &str) -> IResult<&str, (&str, &str)> {
    terminated(separated_pair(digit1, space1, digit1), space0)(line)
}

fn parse_seeds(line: &str) -> Vec<Range> {
    match tag::<&str, &str, Error<_>>("seeds: ")(line) {
        Ok((nums, _)) => iterator(nums, parse_seed_ranges)
                            .map(|(start, length)| Range::new(start.parse().unwrap(), 
                                                              length.parse().unwrap()))
                            .collect(),
        Err(_) => panic!("didn't start with seeds")
    }
}

fn ranges_parser(line: &str) -> (Range, Range) {
    let mut nums = line.split_whitespace().map(|x| x.parse().expect("invalid ranges"));
    let dst_start = nums.next().expect("no dst_start");
    let src_start = nums.next().expect("no src_start");
    let length = nums.next().expect("no length");
    (Range::new(src_start, length), Range::new(dst_start, length))
}

fn parse_description(line: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag("-to-"), alpha1)(line)
}

fn part2(input: &str) -> u128 {
    let mut lines = input.lines();
    let mut maps = BTreeMap::new();

    let mut seeds = parse_seeds(lines.next().expect("empty input")) ;

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
        seeds = seeds.iter().flat_map(|x| mapping.range_query.query(x)).collect();
        k = mapping.dst;
    }

    seeds.iter().map(|range| range.start).min().expect("no minimum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("seeds: 79 14 55 13

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
        assert_eq!(result, 46);
    }

}


