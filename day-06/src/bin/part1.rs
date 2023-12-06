use nom::{error, bytes::complete::tag, sequence::terminated, multi::separated_list1, character::complete::{multispace1, space1, u32}};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_tagged_num_list(t: &str, line: &str) -> Vec<i32>{
    let (nums, _) = terminated(tag(t), multispace1::<&str, error::Error<_>>)(line).expect("line should start with a tag");

    separated_list1(space1::<&str, error::Error<_>>, nom::character::complete::i32)(nums).expect("should be parseable list of nums").1

}

fn calculate_hold_time((t,d): (&i32, &i32)) -> (u32, u32) {
    let tsquared = t.pow(2) as f32;
    let fourac = (4 * d) as f32; 
    let lower = ((-t as f32 + (tsquared - fourac).sqrt())/-2_f32).floor() as u32 + 1;
    let upper = ((*t as f32 + (tsquared - fourac).sqrt())/2_f32).ceil() as u32;
    (lower, upper)
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = parse_tagged_num_list("Time:", lines.next().expect("should have a time line").trim());
    let distances = parse_tagged_num_list("Distance: ", lines.next().expect("should have a destination line").trim());
    times.iter().zip(distances.iter())
                .map(calculate_hold_time)
                .map(|(l, r)| r - l)
                .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("Time:      7  15   30
        Distance:  9  40  200");
        assert_eq!(result, 288);
    }

}


