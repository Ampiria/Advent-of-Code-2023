use std::collections::BTreeMap;
use num::integer::lcm;
use nom::{
    sequence::{separated_pair, delimited},
    character::complete::alphanumeric1,
    bytes::complete::tag
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_line(line: &str) -> (&str, (&str, &str)){
    separated_pair(alphanumeric1::<&str, nom::error::Error<_>>, 
                   tag(" = "),  
                   delimited(tag("("), 
                             separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                             tag(")"))
                   )(line).expect("line should be parseable").1
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let directions = lines.next().expect("starts with directions").chars().collect::<Vec<_>>();
    lines.next(); //get rid of empty line

    let map = lines.map(|line| parse_line(line.trim())).collect::<BTreeMap<_, _>>();

    map.keys()
       .filter(|loc| loc.ends_with('A'))
       .copied()
       .map(|mut loc| {
            directions.iter()
            .cycle()
            .position(|direction| {
                loc = match direction {
                    'L' => map.get(loc).unwrap().0,
                    'R' => map.get(loc).unwrap().1,
                    _ => panic!("invalid instruction {}", direction)
                };
                loc.ends_with('Z')
           }).expect("should be able to reach a z ending") + 1
       }).reduce(lcm).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)");
        assert_eq!(result, 6);
    }
}


