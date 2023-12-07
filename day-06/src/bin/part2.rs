use nom::{error, multi::separated_list1, bytes::complete::tag, sequence::terminated, character::{complete::{multispace1, space1, digit0}}};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_tagged_num(t: &str, line: &str) -> i64{
    let (nums, _) = terminated(tag(t), multispace1::<&str, error::Error<_>>)(line).expect("line should start with a tag");

        separated_list1(space1::<&str, error::Error<_>>, digit0)(nums).expect("should be parseable list of nums").1
            .join("")
            .parse::<i64>().expect("should be a number")
}

//calculate lower and upper limits using quadratic formula such that lower <= hold < upper
fn calculate_hold_time(t: i64 , d: i64) -> (u64, u64) {
    let tsquared = t.pow(2) as f64;
    let fourac = (4 * d) as f64; 
    let lower = ((-t as f64 + (tsquared - fourac).sqrt())/-2_f64).floor() as u64 + 1;
    let upper = ((t as f64 + (tsquared - fourac).sqrt())/2_f64).ceil() as u64;
    (lower, upper)
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = parse_tagged_num("Time:", lines.next().expect("should have a time line").trim());
    let distance = parse_tagged_num("Distance: ", lines.next().expect("should have a destination line").trim());
    let (low, upp) = calculate_hold_time(time, distance);
    upp - low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("Time:      7  15   30
        Distance:  9  40  200");
        assert_eq!(result, 71503);
    }

}


