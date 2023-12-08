use std::collections::BTreeMap;
use nom::{
    sequence::{separated_pair, delimited},
    character::complete::alpha1,
    bytes::complete::tag
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_line(line: &str) -> (&str, (&str, &str)){
    separated_pair(alpha1::<&str, nom::error::Error<_>>, 
                   tag(" = "),  
                   delimited(tag("("), 
                             separated_pair(alpha1, tag(", "), alpha1),
                             tag(")"))
                   )(line).expect("line should be parseable").1
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let directions = lines.next().expect("starts with directions").chars().collect::<Vec<_>>();
    lines.next(); //get rid of empty line

    let map = lines.map(|line| parse_line(line.trim())).collect::<BTreeMap<_, _>>();

    let mut loc = "AAA";
    let mut steps = 0;
    let n = directions.len();
    while loc != "ZZZ" {
        let mapping = map.get(loc).expect("location should be in map");
        loc = match directions[steps % n] {
            'L' => mapping.0,
            'R' => mapping.1,
            _ => panic!("got unexpected direction: {}", directions[steps % n])
        };
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_repeat() {
        let result = part1("LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, 6);
    }
}


